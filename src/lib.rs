#![feature(lazy_cell, ptr_sub_ptr)]
use engage::battle::BattleInfoSide;
use engage::calculator::*;
use unity::{prelude::*};
use engage::gamedata::{unit::*};

#[unity::hook("App", "UnitCalculator", "AddCommand")]
fn add_command_hook(calculator: &mut CalculatorManager, method_info: OptionalMethod) {
    call_original!(calculator, method_info);

    let movement_command: &mut CalculatorCommand = calculator.find_command("幸運");   
    let movement_instance = il2cpp::instantiate_class::<GameCalculatorCommand>(movement_command.get_class().clone()).unwrap();  

    movement_instance.get_class_mut().get_virtual_method_mut("get_Name").map(|method| method.method_ptr = get_movement_name as _);
    movement_instance.get_class_mut().get_virtual_method_mut("GetImpl").map(|method| method.method_ptr = get_movement_stat_unit as _);
    movement_instance.get_class_mut().get_vtable_mut()[31].method_ptr = get_movement_stat_battle_info as *mut u8;

    calculator.add_command(movement_instance); 

    let movement_instance_reverse = il2cpp::instantiate_class::<GameCalculatorCommand>(movement_command.get_class().clone()).unwrap();
    movement_instance_reverse.get_class_mut().get_virtual_method_mut("get_Name").map(|method| method.method_ptr = get_movement_name as _);
    movement_instance_reverse.get_class_mut().get_virtual_method_mut("GetImpl").map(|method| method.method_ptr = get_movement_stat_unit as _);
    movement_instance_reverse.get_class_mut().get_vtable_mut()[31].method_ptr = get_movement_stat_battle_info as *mut u8;

    let reverse_movement_command = movement_instance_reverse.reverse();
    calculator.add_command(reverse_movement_command); 
}

pub fn get_movement_name(_this: &GameCalculatorCommand, _method_info: OptionalMethod) -> &'static Il2CppString {
    "Mov".into()
}

pub fn get_movement_stat_unit(_this: &GameCalculatorCommand, unit: &Unit, _method_info: OptionalMethod) -> f32 {
    unit.get_capability(10, true) as f32
}

pub fn get_movement_stat_battle_info(_this: &GameCalculatorCommand, side: &BattleInfoSide, _method_info: OptionalMethod) -> f32 {
    side.detail.capability.data[10] as f32
}

#[skyline::main(name = "movement_plugin")]
pub fn main() {
    std::panic::set_hook(Box::new(|info| {
        let location = info.location().unwrap();
        let msg = match info.payload().downcast_ref::<&'static str>() {
            Some(s) => *s,
            None => {
                match info.payload().downcast_ref::<String>() {
                    Some(s) => &s[..],
                    None => "Box<Any>",
                }
            },
        };

        let err_msg = format!(
            "Movement plugin has panicked at '{}' with the following message:\n{}\0",
            location,
            msg
        );

        skyline::error::show_error(
            42069,
            "Movement plugin has panicked! Please open the details and send a screenshot to the developer, then close the game.\n\0",
            err_msg.as_str(),
        );
    }));

    skyline::install_hooks!(add_command_hook);
}