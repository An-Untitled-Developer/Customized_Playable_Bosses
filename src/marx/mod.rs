use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash::lua2cpp::L2CFighterCommon;
use smash::app::BattleObjectModuleAccessor;
use smash::phx::Vector3f;
use smash::app::ItemKind;
use smash::app::sv_battle_object;
use std::u32;
use smash::app::FighterUtil;
use smash::app::sv_information;
use smash::app::lua_bind;
use skyline::nn::ro::LookupSymbol;
use smash::phx::Hash40;
use smash::hash40;
use smash::app::utility::get_category;

static mut CONTROLLABLE : bool = true;
static mut ENTRY_ID : usize = 0;
static mut BOSS_ID : [u32; 8] = [0; 8];
pub static mut FIGHTER_MANAGER: usize = 0;
static mut DEAD : bool = false;
static mut JUMP_START : bool = false;
static mut RESULT_SPAWNED : bool = false;
pub static mut FIGHTER_NAME: [u64;4] = [0;4];

pub unsafe fn read_tag(addr: u64) -> String {
    let mut s: Vec<u8> = vec![];

    let mut addr = addr as *const u16;
    loop {
        if *addr == 0_u16 {
            break;
        }
        s.push(*(addr as *const u8));
        addr = addr.offset(1);
    }
    // No null terminator needed

    std::str::from_utf8(&s).unwrap().to_owned()
}

pub unsafe fn get_player_number(module_accessor:  &mut smash::app::BattleObjectModuleAccessor) -> usize {
    let player_number;
    if smash::app::utility::get_kind(module_accessor) == *WEAPON_KIND_PTRAINER_PTRAINER {
        player_number = WorkModule::get_int(module_accessor, *WEAPON_PTRAINER_PTRAINER_INSTANCE_WORK_ID_INT_FIGHTER_ENTRY_ID) as usize;
    }
    else if get_category(module_accessor) == *BATTLE_OBJECT_CATEGORY_FIGHTER {
        player_number = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    }
    else {
        let mut owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
        while get_category(owner_module_accessor) != *BATTLE_OBJECT_CATEGORY_FIGHTER { //Keep checking the owner of the boma we're working with until we've hit a boma that belongs to a fighter
            owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(owner_module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
        }
        player_number = WorkModule::get_int(owner_module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    }
    return player_number;
}

pub fn once_per_fighter_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let lua_state = fighter.lua_state_agent;
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
        let fighter_kind = smash::app::utility::get_kind(module_accessor);
        if fighter_kind == *FIGHTER_KIND_KOOPAG {
            pub unsafe fn entry_id(module_accessor: &mut BattleObjectModuleAccessor) -> usize {
                let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
                return entry_id;
            }
            ENTRY_ID = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
            LookupSymbol(
                &mut FIGHTER_MANAGER,
                "_ZN3lib9SingletonIN3app14FighterManagerEE9instance_E\u{0}"
                .as_bytes()
                .as_ptr(),
            );
            let fighter_manager = *(FIGHTER_MANAGER as *mut *mut smash::app::FighterManager);
            let text = skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as u64;
            let name_base = text + 0x52c3758;
            FIGHTER_NAME[get_player_number(&mut *fighter.module_accessor)] = hash40(&read_tag(name_base + 0x260 * get_player_number(&mut *fighter.module_accessor) as u64 + 0x8e));
            if FIGHTER_NAME[get_player_number(module_accessor)] == hash40("MARX") {
                if sv_information::is_ready_go() == false {
                    DEAD = false;
                    CONTROLLABLE = true;
                    JUMP_START = false;
                    let lua_state = fighter.lua_state_agent;
                    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
                    ENTRY_ID = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
                    if ModelModule::scale(module_accessor) != 0.0001 {
                        RESULT_SPAWNED = false;
                        ItemModule::have_item(module_accessor, ItemKind(*ITEM_KIND_MARX), 0, 0, false, false);
                        BOSS_ID[entry_id(module_accessor)] = ItemModule::get_have_item_id(module_accessor, 0) as u32;
                        let boss_boma = sv_battle_object::module_accessor(BOSS_ID[entry_id(module_accessor)]);
                        ModelModule::set_scale(module_accessor, 0.0001);
                        StatusModule::change_status_request_from_script(boss_boma, *ITEM_STATUS_KIND_FOR_BOSS_START, true);
                    }
                }

                if sv_information::is_ready_go() == true {
                    let boss_boma = sv_battle_object::module_accessor(BOSS_ID[entry_id(module_accessor)]);
                    let attack = WorkModule::get_int(boss_boma, *ITEM_INSTANCE_WORK_INT_ATTACK_KIND);
                    if CONTROLLABLE == true {
                        MotionModule::change_motion(boss_boma,Hash40::new("wait"),0.0,1.0,false,0.0,false,false);
                        if attack != 0 {
                            //boss_private::main_energy_from_param(lua_state,ItemKind(*ITEM_KIND_MASTERHAND),Hash40::new("energy_param_wait"),0.0);
                            StatusModule::change_status_request_from_script(boss_boma,attack,false);
                            WorkModule::set_int(boss_boma, 0, *ITEM_INSTANCE_WORK_INT_ATTACK_KIND);
                        }
                    }
                }

                if ModelModule::scale(module_accessor) == 0.0001 {
                    let boss_boma = sv_battle_object::module_accessor(BOSS_ID[entry_id(module_accessor)]);
                    if StatusModule::status_kind(boss_boma) == *ITEM_STATUS_KIND_ENTRY {
                        MotionModule::set_rate(boss_boma, 5.0);
                        smash::app::lua_bind::ItemMotionAnimcmdModuleImpl::set_fix_rate(boss_boma, 5.0);
                    }
                }

                // FIXES SPAWN

                if sv_information::is_ready_go() == true {
                    if CONTROLLABLE == true {
                        let boss_boma = sv_battle_object::module_accessor(BOSS_ID[entry_id(module_accessor)]);
                        if StatusModule::status_kind(boss_boma) != *ITEM_STATUS_KIND_WAIT {
                            if FighterInformation::is_operation_cpu(FighterManager::get_fighter_information(fighter_manager,smash::app::FighterEntryID(ENTRY_ID as i32))) == true {
                                CONTROLLABLE = false;
                                if StatusModule::status_kind(boss_boma) == *ITEM_STATUS_KIND_ENTRY {
                                    StatusModule::change_status_request_from_script(boss_boma, *ITEM_STATUS_KIND_WAIT, true);
                                }
                            }
                            else {
                                StatusModule::change_status_request_from_script(boss_boma, *ITEM_STATUS_KIND_WAIT, true);
                            }
                        }
                    }
                }
                
                if DEAD == false {
                    if sv_information::is_ready_go() == true {
                        if JUMP_START == false {
                            JUMP_START = true;
                            CONTROLLABLE = false;
                            let boss_boma = sv_battle_object::module_accessor(BOSS_ID[entry_id(module_accessor)]);
                            if lua_bind::PostureModule::lr(boss_boma) == -1.0 { // left
                                let vec3 = Vector3f{x: 0.0, y: 90.0, z: 0.0};
                                PostureModule::set_rot(boss_boma,&vec3,0);
                            }
                            if lua_bind::PostureModule::lr(boss_boma) == 1.0 { // right
                                let vec3 = Vector3f{x: 0.0, y: -90.0, z: 0.0};
                                PostureModule::set_rot(boss_boma,&vec3,0);
                            }
                        }
                    }
                }

                if DEAD == false {
                    if sv_information::is_ready_go() == true {
                        // SET POS AND STOPS OUT OF BOUNDS
                        if ModelModule::scale(module_accessor) == 0.0001 {
                            if StatusModule::status_kind(module_accessor) != *FIGHTER_STATUS_KIND_STANDBY {
                                StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_STANDBY, true);
                            }
                            let boss_boma = sv_battle_object::module_accessor(BOSS_ID[entry_id(module_accessor)]);
                            if StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_STANDBY {
                                let x = PostureModule::pos_x(boss_boma);
                                let y = PostureModule::pos_y(boss_boma);
                                let z = PostureModule::pos_z(boss_boma);
                                let boss_pos = Vector3f{x: x, y: y, z: z};
                                if PostureModule::pos_y(boss_boma) >= 150.0 {
                                    let boss_y_pos_1 = Vector3f{x: x, y: 150.0, z: z};
                                    PostureModule::set_pos(module_accessor, &boss_y_pos_1);
                                    PostureModule::set_pos(boss_boma, &boss_y_pos_1);
                                    if PostureModule::pos_y(boss_boma) <= -100.0 {
                                        let boss_y_pos_2 = Vector3f{x: x, y: -100.0, z: z};
                                        PostureModule::set_pos(module_accessor, &boss_y_pos_2);
                                        PostureModule::set_pos(boss_boma, &boss_y_pos_2);
                                    }
                                    if PostureModule::pos_x(boss_boma) >= 200.0 {
                                        let boss_x_pos_1 = Vector3f{x: 200.0, y: 100.0, z: z};
                                        PostureModule::set_pos(module_accessor, &boss_x_pos_1);
                                        PostureModule::set_pos(boss_boma, &boss_x_pos_1);
                                    }
                                    if PostureModule::pos_x(boss_boma) <= -200.0 {
                                        let boss_x_pos_2 = Vector3f{x: -200.0, y: 100.0, z: z};
                                        PostureModule::set_pos(module_accessor, &boss_x_pos_2);
                                        PostureModule::set_pos(boss_boma, &boss_x_pos_2);
                                    }
                                }
                                else if PostureModule::pos_y(boss_boma) <= -100.0 {
                                    let boss_y_pos_2 = Vector3f{x: x, y: -100.0, z: z};
                                    PostureModule::set_pos(module_accessor, &boss_y_pos_2);
                                    PostureModule::set_pos(boss_boma, &boss_y_pos_2);
                                    if PostureModule::pos_x(boss_boma) >= 200.0 {
                                        let boss_x_pos_1 = Vector3f{x: 200.0, y: -100.0, z: z};
                                        PostureModule::set_pos(module_accessor, &boss_x_pos_1);
                                        PostureModule::set_pos(boss_boma, &boss_x_pos_1);
                                    }
                                    if PostureModule::pos_x(boss_boma) <= -200.0 {
                                        let boss_x_pos_2 = Vector3f{x: -200.0, y: -100.0, z: z};
                                        PostureModule::set_pos(module_accessor, &boss_x_pos_2);
                                        PostureModule::set_pos(boss_boma, &boss_x_pos_2);
                                    }
                                    if PostureModule::pos_y(boss_boma) >= 100.0 {
                                        let boss_y_pos_1 = Vector3f{x: x, y: 100.0, z: z};
                                        PostureModule::set_pos(module_accessor, &boss_y_pos_1);
                                        PostureModule::set_pos(boss_boma, &boss_y_pos_1);
                                    }
                                }
                                else if PostureModule::pos_x(boss_boma) >= 200.0 {
                                    let boss_x_pos_1 = Vector3f{x: 200.0, y: y, z: z};
                                    PostureModule::set_pos(module_accessor, &boss_x_pos_1);
                                    PostureModule::set_pos(boss_boma, &boss_x_pos_1);
                                    if PostureModule::pos_x(boss_boma) <= -200.0 {
                                        let boss_x_pos_2 = Vector3f{x: -200.0, y: y, z: z};
                                        PostureModule::set_pos(module_accessor, &boss_x_pos_2);
                                        PostureModule::set_pos(boss_boma, &boss_x_pos_2);
                                    }
                                    if PostureModule::pos_y(boss_boma) >= 150.0 {
                                        let boss_y_pos_1 = Vector3f{x: x, y: 150.0, z: z};
                                        PostureModule::set_pos(module_accessor, &boss_y_pos_1);
                                        PostureModule::set_pos(boss_boma, &boss_y_pos_1);
                                    }
                                    if PostureModule::pos_y(boss_boma) <= -100.0 {
                                        let boss_y_pos_2 = Vector3f{x: x, y: -100.0, z: z};
                                        PostureModule::set_pos(module_accessor, &boss_y_pos_2);
                                        PostureModule::set_pos(boss_boma, &boss_y_pos_2);
                                    }
                                }
                                else if PostureModule::pos_x(boss_boma) <= -200.0 {
                                    let boss_x_pos_2 = Vector3f{x: -200.0, y: y, z: z};
                                    PostureModule::set_pos(module_accessor, &boss_x_pos_2);
                                    PostureModule::set_pos(boss_boma, &boss_x_pos_2);
                                    if PostureModule::pos_y(boss_boma) >= 100.0 {
                                        let boss_y_pos_1 = Vector3f{x: x, y: 100.0, z: z};
                                        PostureModule::set_pos(module_accessor, &boss_y_pos_1);
                                        PostureModule::set_pos(boss_boma, &boss_y_pos_1);
                                    }
                                    if PostureModule::pos_y(boss_boma) <= -100.0 {
                                        let boss_y_pos_2 = Vector3f{x: x, y: -100.0, z: z};
                                        PostureModule::set_pos(module_accessor, &boss_y_pos_2);
                                        PostureModule::set_pos(boss_boma, &boss_y_pos_2);
                                    }
                                    if PostureModule::pos_x(boss_boma) >= 200.0 {
                                        let boss_x_pos_1 = Vector3f{x: 200.0, y: y, z: z};
                                        PostureModule::set_pos(module_accessor, &boss_x_pos_1);
                                        PostureModule::set_pos(boss_boma, &boss_x_pos_1);
                                    }
                                }
                                PostureModule::set_pos(module_accessor, &boss_pos);
                            }
                        }
                    }
                }

                //DAMAGE MODULES
                
                let boss_boma = sv_battle_object::module_accessor(BOSS_ID[entry_id(module_accessor)]);
                DamageModule::set_damage_lock(boss_boma, true);
                HitModule::set_whole(module_accessor, smash::app::HitStatus(*HIT_STATUS_XLU), 0);

                if StopModule::is_damage(boss_boma) {
                    if FighterUtil::is_hp_mode(module_accessor) == true {
                        if DamageModule::damage(module_accessor, 0) < 1.0 {
                            if DEAD == false {
                                CONTROLLABLE = false;
                                StatusModule::change_status_request_from_script(boss_boma, *ITEM_STATUS_KIND_DEAD,true);
                                DEAD = true;
                            }
                        }
                    }
                    if FighterUtil::is_hp_mode(module_accessor) == false {
                        if DamageModule::damage(module_accessor, 0) >= 359.0 {
                            if DEAD == false {
                                CONTROLLABLE = false;
                                StatusModule::change_status_request_from_script(boss_boma, *ITEM_STATUS_KIND_DEAD,true);
                                DEAD = true;
                            }
                        }
                    }
                    if StatusModule::status_kind(boss_boma) != *ITEM_MARX_STATUS_KIND_ATTACK_FOLLOW_EYE_START {
                        if StatusModule::status_kind(boss_boma) != *ITEM_MARX_STATUS_KIND_ATTACK_FOLLOW_EYE_START {
                            if StatusModule::status_kind(boss_boma) != *ITEM_MARX_STATUS_KIND_ATTACK_FOLLOW_EYE_START {
                                if StatusModule::status_kind(boss_boma) != *ITEM_MARX_STATUS_KIND_ATTACK_FACET_EYE_LASER_START {
                                    if StatusModule::status_kind(boss_boma) != *ITEM_MARX_STATUS_KIND_ATTACK_FACET_EYE_LASER {
                                        if StatusModule::status_kind(boss_boma) != *ITEM_MARX_STATUS_KIND_ATTACK_FACET_EYE_LASER_END {
                                            if StatusModule::status_kind(boss_boma) != *ITEM_MARX_STATUS_KIND_ATTACK_FOLLOW_EYE_END {
                                                if StatusModule::status_kind(boss_boma) != *ITEM_MARX_STATUS_KIND_ATTACK_FOLLOW_EYE_LOOP {
                                                    DamageModule::add_damage(module_accessor, 4.1, 0);
                                                    if StopModule::is_stop(module_accessor) {
                                                        StopModule::end_stop(module_accessor);
                                                    }
                                                    if StopModule::is_stop(boss_boma) {
                                                        StopModule::end_stop(boss_boma);
                                                    }
                                                }
                                                else {
                                                    DamageModule::add_damage(module_accessor, 0.1, 0);
                                                }
                                            }
                                            else {
                                                DamageModule::add_damage(module_accessor, 0.1, 0);
                                            }
                                        }
                                        else {
                                            DamageModule::add_damage(module_accessor, 0.1, 0);
                                        }
                                    }
                                    else {
                                        DamageModule::add_damage(module_accessor, 0.1, 0);
                                    }
                                }
                                else {
                                    DamageModule::add_damage(module_accessor, 0.1, 0);
                                }
                            }
                            else {
                                DamageModule::add_damage(module_accessor, 0.1, 0);
                            }
                        }
                        else {
                            DamageModule::add_damage(module_accessor, 0.1, 0);
                        }
                    }
                    else {
                        DamageModule::add_damage(module_accessor, 0.1, 0);
                    }
                }

                if DEAD == true {
                    if sv_information::is_ready_go() == true {
                        if MotionModule::frame(boss_boma) >= MotionModule::end_frame(boss_boma) {
                            StatusModule::change_status_request_from_script(boss_boma, *ITEM_STATUS_KIND_STANDBY, true);
                        }
                    }
                }

                // DEATH CHECK
                if DEAD == true {
                    if sv_information::is_ready_go() == true {
                        if FighterInformation::stock_count(FighterManager::get_fighter_information(fighter_manager,smash::app::FighterEntryID(ENTRY_ID as i32))) != 0 {
                            if StatusModule::status_kind(boss_boma) == *ITEM_STATUS_KIND_STANDBY {
                                StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_DEAD,true);
                            }
                        }
                    }
                }

                let fighter_manager = *(FIGHTER_MANAGER as *mut *mut smash::app::FighterManager);
                if FighterManager::is_result_mode(fighter_manager) == true {
                    if RESULT_SPAWNED == false {
                        RESULT_SPAWNED = true;
                        ItemModule::have_item(module_accessor, ItemKind(*ITEM_KIND_MARX), 0, 0, false, false);
                        BOSS_ID[entry_id(module_accessor)] = ItemModule::get_have_item_id(module_accessor, 0) as u32;
                        let boss_boma = sv_battle_object::module_accessor(BOSS_ID[entry_id(module_accessor)]);
                        StatusModule::change_status_request_from_script(boss_boma, *ITEM_STATUS_KIND_FOR_BOSS_START,true);
                    }
                }

                // SETS POWER

                AttackModule::set_power_mul(boss_boma, 1.5);

                if FighterInformation::is_operation_cpu(FighterManager::get_fighter_information(fighter_manager,smash::app::FighterEntryID(ENTRY_ID as i32))) == false {
                    if StatusModule::status_kind(boss_boma) == *ITEM_STATUS_KIND_WAIT {
                        CONTROLLABLE = true;
                    }
                    if StatusModule::status_kind(boss_boma) == *ITEM_MARX_STATUS_KIND_MOVE_STRAIGHT {
                        CONTROLLABLE = true;
                    }
                    if StatusModule::status_kind(boss_boma) == *ITEM_STATUS_KIND_STANDBY {
                        CONTROLLABLE = true;
                        StatusModule::change_status_request_from_script(boss_boma, *ITEM_STATUS_KIND_WAIT, true);
                    }

                    if StatusModule::status_kind(boss_boma) == *ITEM_MARX_STATUS_KIND_MOVE_TELEPORT {
                        CONTROLLABLE = false;
                        if MotionModule::frame(boss_boma) == 22.0 {
                            CONTROLLABLE = true;
                        }
                    }
                    if StatusModule::status_kind(boss_boma) == *ITEM_MARX_STATUS_KIND_ATTACK_PLANT_GROWS {
                        CONTROLLABLE = false;
                        if MotionModule::frame(boss_boma) == 20.0 {
                            CONTROLLABLE = true;
                        }
                    }
                    if StatusModule::status_kind(boss_boma) == *ITEM_STATUS_KIND_WARP {
                        CONTROLLABLE = false;
                        if MotionModule::frame(boss_boma) == 22.0 {
                            CONTROLLABLE = true;
                        }
                    }
                    if StatusModule::status_kind(boss_boma) == *ITEM_MARX_STATUS_KIND_AVOID_TELEPORT {
                        CONTROLLABLE = false;
                        if MotionModule::frame(boss_boma) == 22.0 {
                            CONTROLLABLE = true;
                        }
                    }
                    if StatusModule::status_kind(boss_boma) == *ITEM_MARX_STATUS_KIND_ATTACK_FACET_EYE_LASER_END {
                        if MotionModule::frame(boss_boma) == 27.0 {
                            CONTROLLABLE = true;
                        }
                    }
                    if StatusModule::status_kind(boss_boma) == *ITEM_MARX_STATUS_KIND_ATTACK_FLY_OUT {
                        if MotionModule::frame(boss_boma) == 27.0 {
                            CONTROLLABLE = true;
                        }
                    }
                    if StatusModule::status_kind(boss_boma) == *ITEM_MARX_STATUS_KIND_ATTACK_FLY_OUT_HOMING {
                        //Boss Control Stick Movement
                        if ControlModule::get_stick_x(module_accessor) <= 0.001 {
                            let pos = Vector3f{x: ControlModule::get_stick_x(module_accessor) * 1.75, y: 0.0, z: 0.0};
                            PostureModule::add_pos(boss_boma, &pos);
                        }
                    
                        if ControlModule::get_stick_x(module_accessor) >= -0.001 {
                            let pos = Vector3f{x: ControlModule::get_stick_x(module_accessor) * 1.75, y: 0.0, z: 0.0};
                            PostureModule::add_pos(boss_boma, &pos);
                        }
                        if ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) == true {
                            StatusModule::change_status_request_from_script(boss_boma, *ITEM_MARX_STATUS_KIND_ATTACK_FLY_OUT, true);
                        }
                    }
                    if StatusModule::status_kind(boss_boma) == *ITEM_MARX_STATUS_KIND_ATTACK_THICK_LASER_END {
                        if MotionModule::frame(boss_boma) == 74.0 {
                            CONTROLLABLE = true;
                        }
                    }
                    if StatusModule::status_kind(boss_boma) == *ITEM_MARX_STATUS_KIND_ATTACK_BLACK_HOLE_END {
                        if MotionModule::frame(boss_boma) == 14.0 {
                            CONTROLLABLE = true;
                        }
                    }
                    if StatusModule::status_kind(boss_boma) == *ITEM_MARX_STATUS_KIND_ATTACK_CAPILLARY_END {
                        if MotionModule::frame(boss_boma) == 10.0 {
                            CONTROLLABLE = true;
                        }
                    }
                    if StatusModule::status_kind(boss_boma) == *ITEM_MARX_STATUS_KIND_ATTACK_FOLLOW_EYE_END {
                        if MotionModule::frame(boss_boma) == 10.0 {
                            CONTROLLABLE = true;
                        }
                    }
                    if StatusModule::status_kind(boss_boma) == *ITEM_MARX_STATUS_KIND_ATTACK_ICE_BOMB_SHOOT {
                        if MotionModule::frame(boss_boma) == 22.0 {
                            CONTROLLABLE = true;
                        }
                    }
                    if StatusModule::status_kind(boss_boma) == *ITEM_MARX_STATUS_KIND_ATTACK_4_CUTTER {
                        if MotionModule::frame(boss_boma) == 34.0 {
                            CONTROLLABLE = true;
                        }
                    }
                    if StatusModule::status_kind(boss_boma) == *ITEM_MARX_STATUS_KIND_ATTACK_4_CUTTER {
                        //Boss Control Stick Movement
                        if ControlModule::get_stick_x(module_accessor) <= 0.001 {
                            let pos = Vector3f{x: ControlModule::get_stick_x(module_accessor) * 0.75, y: 0.0, z: 0.0};
                            PostureModule::add_pos(boss_boma, &pos);
                        }
                    
                        if ControlModule::get_stick_x(module_accessor) >= -0.001 {
                            let pos = Vector3f{x: ControlModule::get_stick_x(module_accessor) * 0.75, y: 0.0, z: 0.0};
                            PostureModule::add_pos(boss_boma, &pos);
                        }
                    
                        if ControlModule::get_stick_y(module_accessor) <= 0.001 {
                            let pos = Vector3f{x: 0.0, y: ControlModule::get_stick_y(module_accessor) * 0.75, z: 0.0};
                            PostureModule::add_pos(boss_boma, &pos);
                        }
                    
                        if ControlModule::get_stick_y(module_accessor) >= -0.001 {
                            let pos = Vector3f{x: 0.0, y: ControlModule::get_stick_y(module_accessor) * 0.75, z: 0.0};
                            PostureModule::add_pos(boss_boma, &pos);
                        }
                    }
                    if StatusModule::status_kind(boss_boma) == *ITEM_MARX_STATUS_KIND_ATTACK_ICE_BOMB_LOOP {
                        //Boss Control Stick Movement
                        if ControlModule::get_stick_x(module_accessor) <= 0.001 {
                            let pos = Vector3f{x: ControlModule::get_stick_x(module_accessor) * 1.0, y: 0.0, z: 0.0};
                            PostureModule::add_pos(boss_boma, &pos);
                        }
                    
                        if ControlModule::get_stick_x(module_accessor) >= -0.001 {
                            let pos = Vector3f{x: ControlModule::get_stick_x(module_accessor) * 1.0, y: 0.0, z: 0.0};
                            PostureModule::add_pos(boss_boma, &pos);
                        }
                    
                        if ControlModule::get_stick_y(module_accessor) <= 0.001 {
                            let pos = Vector3f{x: 0.0, y: ControlModule::get_stick_y(module_accessor) * 1.0, z: 0.0};
                            PostureModule::add_pos(boss_boma, &pos);
                        }
                    
                        if ControlModule::get_stick_y(module_accessor) >= -0.001 {
                            let pos = Vector3f{x: 0.0, y: ControlModule::get_stick_y(module_accessor) * 1.0, z: 0.0};
                            PostureModule::add_pos(boss_boma, &pos);
                        }
                    }
                    if StatusModule::status_kind(boss_boma) == *ITEM_MARX_STATUS_KIND_ATTACK_THICK_LASER_LOOP {
                        //Boss Control Stick Movement
                        if ControlModule::get_stick_x(module_accessor) <= 0.001 {
                            let pos = Vector3f{x: ControlModule::get_stick_x(module_accessor) * 1.0, y: 0.0, z: 0.0};
                            PostureModule::add_pos(boss_boma, &pos);
                        }
                    
                        if ControlModule::get_stick_x(module_accessor) >= -0.001 {
                            let pos = Vector3f{x: ControlModule::get_stick_x(module_accessor) * 1.0, y: 0.0, z: 0.0};
                            PostureModule::add_pos(boss_boma, &pos);
                        }
                    
                        if ControlModule::get_stick_y(module_accessor) <= 0.001 {
                            let pos = Vector3f{x: 0.0, y: ControlModule::get_stick_y(module_accessor) * 1.0, z: 0.0};
                            PostureModule::add_pos(boss_boma, &pos);
                        }
                    
                        if ControlModule::get_stick_y(module_accessor) >= -0.001 {
                            let pos = Vector3f{x: 0.0, y: ControlModule::get_stick_y(module_accessor) * 1.0, z: 0.0};
                            PostureModule::add_pos(boss_boma, &pos);
                        }
                    }

                    if CONTROLLABLE == true {
                        if DEAD == false {
                            //Boss Control Stick Movement
                            if ControlModule::get_stick_x(module_accessor) <= 0.001 {
                                let pos = Vector3f{x: ControlModule::get_stick_x(module_accessor) * 2.2, y: 0.0, z: 0.0};
                                PostureModule::add_pos(boss_boma, &pos);
                            }
                        
                            if ControlModule::get_stick_x(module_accessor) >= -0.001 {
                                let pos = Vector3f{x: ControlModule::get_stick_x(module_accessor) * 2.2, y: 0.0, z: 0.0};
                                PostureModule::add_pos(boss_boma, &pos);
                            }
                        
                            if ControlModule::get_stick_y(module_accessor) <= 0.001 {
                                let pos = Vector3f{x: 0.0, y: ControlModule::get_stick_y(module_accessor) * 2.2, z: 0.0};
                                PostureModule::add_pos(boss_boma, &pos);
                            }
                        
                            if ControlModule::get_stick_y(module_accessor) >= -0.001 {
                                let pos = Vector3f{x: 0.0, y: ControlModule::get_stick_y(module_accessor) * 2.2, z: 0.0};
                                PostureModule::add_pos(boss_boma, &pos);
                            }
                        
                            //Boss Moves
                            if ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
                                CONTROLLABLE = false;
                                StatusModule::change_status_request_from_script(boss_boma, *ITEM_MARX_STATUS_KIND_ATTACK_PLANT_START, true);
                            }
                            if ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_JUMP) {
                                CONTROLLABLE = false;
                                StatusModule::change_status_request_from_script(boss_boma, *ITEM_MARX_STATUS_KIND_ATTACK_4_CUTTER, true);
                            }
                            if ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_GUARD) {
                                CONTROLLABLE = false;
                                StatusModule::change_status_request_from_script(boss_boma, *ITEM_MARX_STATUS_KIND_MOVE_TELEPORT, true);
                            }
                            if ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
                                CONTROLLABLE = false;
                                StatusModule::change_status_request_from_script(boss_boma, *ITEM_MARX_STATUS_KIND_ATTACK_ICE_BOMB_START, true);
                            }
                            if ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW != 0 {
                                CONTROLLABLE = false;
                                StatusModule::change_status_request_from_script(boss_boma, *ITEM_MARX_STATUS_KIND_ATTACK_CAPILLARY_START, true);
                            }
                            if ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_HI != 0 {
                                CONTROLLABLE = false;
                                StatusModule::change_status_request_from_script(boss_boma, *ITEM_MARX_STATUS_KIND_ATTACK_THICK_LASER_START, true);
                            }
                            if ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_S != 0 {
                                CONTROLLABLE = false;
                                StatusModule::change_status_request_from_script(boss_boma, *ITEM_MARX_STATUS_KIND_ATTACK_FACET_EYE_LASER_START, true);
                            }
                            if ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW3 != 0 {
                                CONTROLLABLE = false;
                                StatusModule::change_status_request_from_script(boss_boma, *ITEM_MARX_STATUS_KIND_ATTACK_BLACK_HOLE_READY, true);
                            }
                            if ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI3 != 0 {
                                CONTROLLABLE = false;
                                StatusModule::change_status_request_from_script(boss_boma, *ITEM_MARX_STATUS_KIND_ATTACK_FOLLOW_EYE_START, true);
                            }
                            if ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S3 != 0 {
                                CONTROLLABLE = false;
                                StatusModule::change_status_request_from_script(boss_boma, *ITEM_MARX_STATUS_KIND_ATTACK_FLY_OUT_HOMING, true);
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn install() {
    acmd::add_custom_hooks!(once_per_fighter_frame);
}