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
use smash::hash40;
use smash::app::utility::get_category;

static mut CONTROLLABLE : bool = true;
static mut GROUNDED : bool = true;
static mut ENTRY_ID : usize = 0;
static mut BOSS_ID : [u32; 8] = [0; 8];
pub static mut FIGHTER_MANAGER: usize = 0;
static mut DEAD : bool = false;
static mut JUMP_START : bool = false;
static mut RESULT_SPAWNED : bool = false;
pub static mut FIGHTER_NAME: [u64;9] = [0;9];
static mut STOP : bool = false;
static mut STUNNED : bool = false;
static mut FINALIZED_STUN : bool = false;
static mut FRESH_CONTROL : bool = false;
static mut EXISTS_PUBLIC : bool = false;

pub unsafe fn check_status() -> bool {
    return EXISTS_PUBLIC;
}

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
        if fighter_kind == *FIGHTER_KIND_MARIO {
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
            if FIGHTER_NAME[get_player_number(module_accessor)] == hash40("RATHALOS")
            || FIGHTER_NAME[get_player_number(module_accessor)] == hash40("リオレウス")
            || FIGHTER_NAME[get_player_number(module_accessor)] == hash40("雄火龙")
            || FIGHTER_NAME[get_player_number(module_accessor)] == hash40("雄火龍")
            || FIGHTER_NAME[get_player_number(module_accessor)] == hash40("리오레우스")
            || FIGHTER_NAME[get_player_number(module_accessor)] == hash40("РАТАЛОС") {
                if smash::app::stage::get_stage_id() == 0x139 {
                    let lua_state = fighter.lua_state_agent;
                    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
                    if ModelModule::scale(module_accessor) != 0.0001 {
                        ItemModule::remove_all(module_accessor);
                        ItemModule::have_item(module_accessor, ItemKind(*ITEM_KIND_LIOLEUSBOSS), 0, 0, false, false);
                        BOSS_ID[entry_id(module_accessor)] = ItemModule::get_have_item_id(module_accessor, 0) as u32;
                        ModelModule::set_scale(module_accessor, 0.0001);
                        let boss_boma = sv_battle_object::module_accessor(BOSS_ID[entry_id(module_accessor)]);
                        ModelModule::set_scale(boss_boma, 0.04);
                        MotionModule::change_motion(boss_boma,smash::phx::Hash40::new("hovering_move"),0.0,1.0,false,0.0,false,false);
                    }
                    if ModelModule::scale(module_accessor) == 0.0001 {
                        MotionModule::change_motion(module_accessor,smash::phx::Hash40::new("none"),0.0,1.0,false,0.0,false,false);
                        ModelModule::set_joint_rotate(module_accessor, smash::phx::Hash40::new("root") , &mut Vector3f{x: -270.0, y: 180.0, z: -90.0}, smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_BEFORE as u8}, ModelModule::rotation_order(module_accessor));
                    }
                }
                else if smash::app::stage::get_stage_id() != 0x13A {
                    if sv_information::is_ready_go() == false {
                        DEAD = false;
                        CONTROLLABLE = true;
                        JUMP_START = false;
                        GROUNDED = true;
                        STOP = false;
                        STUNNED = false;
                        FINALIZED_STUN = false;
                        FRESH_CONTROL = false;
                        let lua_state = fighter.lua_state_agent;
                        let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
                        ENTRY_ID = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
                        if ModelModule::scale(module_accessor) != 0.0001 {
                            EXISTS_PUBLIC = true;
                            RESULT_SPAWNED = false;
                            ItemModule::have_item(module_accessor, ItemKind(*ITEM_KIND_LIOLEUSBOSS), 0, 0, false, false);
                            BOSS_ID[entry_id(module_accessor)] = ItemModule::get_have_item_id(module_accessor, 0) as u32;
                            let boss_boma = sv_battle_object::module_accessor(BOSS_ID[entry_id(module_accessor)]);
                            ModelModule::set_scale(module_accessor, 0.0001);
                            StatusModule::change_status_request_from_script(boss_boma, *ITEM_STATUS_KIND_FOR_BOSS_START, true);
                        }
                    }

                    if sv_information::is_ready_go() == false {
                        if StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_ENTRY {
                            FighterManager::set_cursor_whole(fighter_manager,false);
                            ArticleModule::set_visibility_whole(module_accessor, *FIGHTER_MARIO_GENERATE_ARTICLE_PUMP, false, smash::app::ArticleOperationTarget(0));
                            StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_WAIT, true);
                        }
                    }

                    //STUBS AI

                    if sv_information::is_ready_go() == true {
                        if FighterInformation::is_operation_cpu(FighterManager::get_fighter_information(fighter_manager,smash::app::FighterEntryID(ENTRY_ID as i32))) == false && DEAD == false {
                            let boss_boma = sv_battle_object::module_accessor(BOSS_ID[entry_id(module_accessor)]);
                            if CONTROLLABLE == true {
                                if GROUNDED == true {
                                    if MotionModule::motion_kind(boss_boma) != smash::hash40("wait") && StatusModule::status_kind(boss_boma) != *ITEM_LIOLEUSBOSS_STATUS_KIND_WAIT {
                                        MotionModule::set_rate(boss_boma, 1.0);
                                        smash::app::lua_bind::ItemMotionAnimcmdModuleImpl::set_fix_rate(boss_boma, 1.0);
                                        if FRESH_CONTROL {
                                            StatusModule::change_status_request_from_script(boss_boma, *ITEM_LIOLEUSBOSS_STATUS_KIND_WAIT, true);
                                            FRESH_CONTROL = false;
                                        }
                                        MotionModule::change_motion(boss_boma,smash::phx::Hash40::new("wait"),0.0,1.0,false,0.0,false,false);
                                    }
                                }
                                else {
                                    if ControlModule::get_stick_x(module_accessor) > -0.1 && ControlModule::get_stick_x(module_accessor) < 0.1
                                    && MotionModule::motion_kind(boss_boma) != smash::hash40("hovering") {
                                        MotionModule::set_rate(boss_boma, 1.0);
                                        smash::app::lua_bind::ItemMotionAnimcmdModuleImpl::set_fix_rate(boss_boma, 1.0);
                                        StatusModule::change_status_request_from_script(boss_boma, *ITEM_LIOLEUSBOSS_STATUS_KIND_DEBUG_CONTROL, true);
                                        MotionModule::change_motion(boss_boma,smash::phx::Hash40::new("hovering"),0.0,1.0,false,0.0,false,false);
                                    }
                                    else if ControlModule::get_stick_x(module_accessor) < -0.1
                                    && MotionModule::motion_kind(boss_boma) != smash::hash40("hovering_move")
                                    && lua_bind::PostureModule::lr(boss_boma) == -1.0 {
                                        MotionModule::set_rate(boss_boma, 1.0);
                                        smash::app::lua_bind::ItemMotionAnimcmdModuleImpl::set_fix_rate(boss_boma, 1.0);
                                        StatusModule::change_status_request_from_script(boss_boma, *ITEM_LIOLEUSBOSS_STATUS_KIND_DEBUG_CONTROL, true);
                                        MotionModule::change_motion(boss_boma,smash::phx::Hash40::new("hovering_move"),0.0,1.0,false,0.0,false,false);
                                    }
                                    else if ControlModule::get_stick_x(module_accessor) > 0.1
                                    && MotionModule::motion_kind(boss_boma) != smash::hash40("hovering_move")
                                    && lua_bind::PostureModule::lr(boss_boma) == 1.0 {
                                        MotionModule::set_rate(boss_boma, 1.0);
                                        smash::app::lua_bind::ItemMotionAnimcmdModuleImpl::set_fix_rate(boss_boma, 1.0);
                                        StatusModule::change_status_request_from_script(boss_boma, *ITEM_LIOLEUSBOSS_STATUS_KIND_DEBUG_CONTROL, true);
                                        MotionModule::change_motion(boss_boma,smash::phx::Hash40::new("hovering_move"),0.0,1.0,false,0.0,false,false);
                                    }
                                }
                            }
                            if CONTROLLABLE == false {
                                if MotionModule::motion_kind(boss_boma) == smash::hash40("wait") {
                                    CONTROLLABLE = true;
                                }
                                if MotionModule::motion_kind(boss_boma) == smash::hash40("hovering") {
                                    CONTROLLABLE = true;
                                }
                            }
                        }
                    }

                    if ModelModule::scale(module_accessor) == 0.0001 {
                        let boss_boma = sv_battle_object::module_accessor(BOSS_ID[entry_id(module_accessor)]);
                        if StatusModule::status_kind(boss_boma) == *ITEM_STATUS_KIND_ENTRY {
                            MotionModule::set_rate(boss_boma, 2.75);
                            smash::app::lua_bind::ItemMotionAnimcmdModuleImpl::set_fix_rate(boss_boma, 2.75);
                        }
                    }

                    if DEAD == false {
                        if sv_information::is_ready_go() == true {
                            // SET POS AND STOPS OUT OF BOUNDS
                            if ModelModule::scale(module_accessor) == 0.0001 {
                                let boss_boma = sv_battle_object::module_accessor(BOSS_ID[entry_id(module_accessor)]);
                                if StatusModule::status_kind(module_accessor) != *FIGHTER_STATUS_KIND_STANDBY {
                                    FighterManager::set_cursor_whole(fighter_manager,false);
                                    fighter.set_situation(SITUATION_KIND_AIR.into());
                                    MotionModule::change_motion(module_accessor,smash::phx::Hash40::new("fall"),0.0,1.0,false,0.0,false,false);
                                }
                                if FighterUtil::is_hp_mode(module_accessor) == true {
                                    if StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_DEAD {
                                        if DEAD == false {
                                            CONTROLLABLE = false;
                                            EXISTS_PUBLIC = false;
                                            StatusModule::change_status_request_from_script(boss_boma, *ITEM_STATUS_KIND_DEAD, true);
                                            DEAD = true;
                                        }
                                    }
                                }
                                let x = PostureModule::pos_x(boss_boma);
                                let y = PostureModule::pos_y(boss_boma);
                                let z = PostureModule::pos_z(boss_boma);
                                let boss_pos = Vector3f{x: x, y: y + 20.0, z: z};
                                if CONTROLLABLE {
                                    if PostureModule::pos_y(boss_boma) <= -100.0 {
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
                                        if PostureModule::pos_y(boss_boma) >= 100.0 {
                                            let boss_y_pos_1 = Vector3f{x: 200.0, y: 100.0, z: z};
                                            PostureModule::set_pos(module_accessor, &boss_y_pos_1);
                                            PostureModule::set_pos(boss_boma, &boss_y_pos_1);
                                        }
                                        if PostureModule::pos_y(boss_boma) <= -50.0 {
                                            let boss_y_pos_2 = Vector3f{x: 200.0, y: -50.0, z: z};
                                            PostureModule::set_pos(module_accessor, &boss_y_pos_2);
                                            PostureModule::set_pos(boss_boma, &boss_y_pos_2);
                                        }
                                    }
                                    else if PostureModule::pos_x(boss_boma) <= -200.0 {
                                        let boss_x_pos_2 = Vector3f{x: -200.0, y: y, z: z};
                                        PostureModule::set_pos(module_accessor, &boss_x_pos_2);
                                        PostureModule::set_pos(boss_boma, &boss_x_pos_2);
                                        if PostureModule::pos_y(boss_boma) >= 100.0 {
                                            let boss_y_pos_1 = Vector3f{x: -200.0, y: 100.0, z: z};
                                            PostureModule::set_pos(module_accessor, &boss_y_pos_1);
                                            PostureModule::set_pos(boss_boma, &boss_y_pos_1);
                                        }
                                        if PostureModule::pos_y(boss_boma) <= -50.0 {
                                            let boss_y_pos_2 = Vector3f{x: -200.0, y: -50.0, z: z};
                                            PostureModule::set_pos(module_accessor, &boss_y_pos_2);
                                            PostureModule::set_pos(boss_boma, &boss_y_pos_2);
                                        }
                                        if PostureModule::pos_x(boss_boma) >= 200.0 {
                                            let boss_x_pos_1 = Vector3f{x: 200.0, y: y, z: z};
                                            PostureModule::set_pos(module_accessor, &boss_x_pos_1);
                                            PostureModule::set_pos(boss_boma, &boss_x_pos_1);
                                        }
                                    }
                                    else {
                                        PostureModule::set_pos(module_accessor, &boss_pos);
                                    }
                                }
                                else {
                                    if PostureModule::pos_y(boss_boma) <= -100.0 {
                                        let boss_y_pos_2 = Vector3f{x: x, y: -100.0, z: z};
                                        PostureModule::set_pos(module_accessor, &boss_y_pos_2);
                                        if PostureModule::pos_x(boss_boma) >= 200.0 {
                                            let boss_x_pos_1 = Vector3f{x: 200.0, y: -100.0, z: z};
                                            PostureModule::set_pos(module_accessor, &boss_x_pos_1);
                                        }
                                        if PostureModule::pos_x(boss_boma) <= -200.0 {
                                            let boss_x_pos_2 = Vector3f{x: -200.0, y: -100.0, z: z};
                                            PostureModule::set_pos(module_accessor, &boss_x_pos_2);
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
                                        if PostureModule::pos_y(boss_boma) >= 100.0 {
                                            let boss_y_pos_1 = Vector3f{x: 200.0, y: 100.0, z: z};
                                            PostureModule::set_pos(module_accessor, &boss_y_pos_1);
                                        }
                                        if PostureModule::pos_y(boss_boma) <= -50.0 {
                                            let boss_y_pos_2 = Vector3f{x: 200.0, y: -50.0, z: z};
                                            PostureModule::set_pos(module_accessor, &boss_y_pos_2);
                                        }
                                    }
                                    else if PostureModule::pos_x(boss_boma) <= -200.0 {
                                        let boss_x_pos_2 = Vector3f{x: -200.0, y: y, z: z};
                                        PostureModule::set_pos(module_accessor, &boss_x_pos_2);
                                        PostureModule::set_pos(boss_boma, &boss_x_pos_2);
                                        if PostureModule::pos_y(boss_boma) >= 100.0 {
                                            let boss_y_pos_1 = Vector3f{x: -200.0, y: 100.0, z: z};
                                            PostureModule::set_pos(module_accessor, &boss_y_pos_1);
                                        }
                                        if PostureModule::pos_y(boss_boma) <= -50.0 {
                                            let boss_y_pos_2 = Vector3f{x: -200.0, y: -50.0, z: z};
                                            PostureModule::set_pos(module_accessor, &boss_y_pos_2);
                                        }
                                        if PostureModule::pos_x(boss_boma) >= 200.0 {
                                            let boss_x_pos_1 = Vector3f{x: 200.0, y: y, z: z};
                                            PostureModule::set_pos(boss_boma, &boss_x_pos_1);
                                        }
                                    }
                                    else {
                                        PostureModule::set_pos(module_accessor, &boss_pos);
                                    }
                                }
                            }
                        }
                    }

                    //DAMAGE MODULES
                    
                    let boss_boma = sv_battle_object::module_accessor(BOSS_ID[entry_id(module_accessor)]);
                    DamageModule::set_damage_lock(boss_boma, true);
                    HitModule::set_whole(module_accessor, smash::app::HitStatus(*HIT_STATUS_XLU), 0);
                    HitModule::set_whole(boss_boma, smash::app::HitStatus(*HIT_STATUS_NORMAL), 0);

                    if sv_information::is_ready_go() == true {
                        if DamageModule::damage(module_accessor, 0) >= 300.0 && !STUNNED && !DEAD && !FINALIZED_STUN {
                            CONTROLLABLE = false;
                            STUNNED = true;
                            StatusModule::change_status_request_from_script(boss_boma,*ITEM_LIOLEUSBOSS_STATUS_KIND_DOWN_START,true);
                        }
                        if DEAD == false {
                            if FighterUtil::is_hp_mode(module_accessor) == false {
                                if DamageModule::damage(module_accessor, 0) >= 599.0 {
                                    CONTROLLABLE = false;
                                    StatusModule::change_status_request_from_script(boss_boma, *ITEM_STATUS_KIND_DEAD,true);
                                    DEAD = true;
                                }
                            }
                            if StopModule::is_damage(boss_boma) | StopModule::is_damage(module_accessor) {
                                DamageModule::add_damage(module_accessor, 4.1, 0);
                                if StopModule::is_stop(module_accessor) {
                                    StopModule::end_stop(module_accessor);
                                }
                                if StopModule::is_stop(boss_boma) {
                                    StopModule::end_stop(boss_boma);
                                }
                            }
                        }
                    }

                    //DEATH CHECK

                    if sv_information::is_ready_go() == true {
                        if DEAD == true {
                            HitModule::set_whole(module_accessor, smash::app::HitStatus(*HIT_STATUS_OFF), 0);
                            let boss_boma = sv_battle_object::module_accessor(BOSS_ID[entry_id(module_accessor)]);
                            HitModule::set_whole(boss_boma, smash::app::HitStatus(*HIT_STATUS_OFF), 0);
                            if STOP == false {
                                if FighterInformation::stock_count(FighterManager::get_fighter_information(fighter_manager,smash::app::FighterEntryID(ENTRY_ID as i32))) != 0 {
                                    StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_DEAD,true);
                                }
                                if FighterInformation::stock_count(FighterManager::get_fighter_information(fighter_manager,smash::app::FighterEntryID(ENTRY_ID as i32))) == 0 {
                                    StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_DEAD,true);
                                    STOP = true;
                                }
                            }
                            if STOP == true {
                                if StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_REBIRTH {
                                    StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_STANDBY,true);
                                }
                            }
                        }
                    }

                    let fighter_manager = *(FIGHTER_MANAGER as *mut *mut smash::app::FighterManager);
                    if FighterManager::is_result_mode(fighter_manager) == true {
                        EXISTS_PUBLIC = false;
                        if RESULT_SPAWNED == false {
                            RESULT_SPAWNED = true;
                            ItemModule::have_item(module_accessor, ItemKind(*ITEM_KIND_LIOLEUSBOSS), 0, 0, false, false);
                            BOSS_ID[entry_id(module_accessor)] = ItemModule::get_have_item_id(module_accessor, 0) as u32;
                            let boss_boma = sv_battle_object::module_accessor(BOSS_ID[entry_id(module_accessor)]);
                            StatusModule::change_status_request_from_script(boss_boma, *ITEM_STATUS_KIND_FOR_BOSS_START,true);
                        }
                    }

                    // SETS POWER

                    AttackModule::set_power_mul(boss_boma, 2.25);

                    // FIXES SPAWN

                    if DEAD == false {
                        if sv_information::is_ready_go() == true {
                            if JUMP_START == false {
                                JUMP_START = true;
                                MotionModule::set_rate(boss_boma, 1.0);
                                smash::app::lua_bind::ItemMotionAnimcmdModuleImpl::set_fix_rate(boss_boma, 1.0);
                                if FighterInformation::is_operation_cpu(FighterManager::get_fighter_information(fighter_manager,smash::app::FighterEntryID(ENTRY_ID as i32))) == false {
                                    MotionModule::change_motion(boss_boma,smash::phx::Hash40::new("wait"),0.0,1.0,false,0.0,false,false);
                                }
                            }
                        }
                    }
                    if sv_information::is_ready_go() == true {
                        if CONTROLLABLE == true {
                            if FighterInformation::is_operation_cpu(FighterManager::get_fighter_information(fighter_manager,smash::app::FighterEntryID(ENTRY_ID as i32))) == true {
                                CONTROLLABLE = false;
                                StatusModule::change_status_request_from_script(boss_boma, *ITEM_LIOLEUSBOSS_STATUS_KIND_ATTACK_HOWLING, true);
                            }
                        }
                    }

                    if FighterInformation::is_operation_cpu(FighterManager::get_fighter_information(fighter_manager,smash::app::FighterEntryID(ENTRY_ID as i32))) == false && DEAD == false && STUNNED {
                        if StatusModule::status_kind(boss_boma) == *ITEM_LIOLEUSBOSS_STATUS_KIND_DOWN_END {
                            if MotionModule::frame(boss_boma) >= MotionModule::end_frame(boss_boma) - 2.0 {
                                STUNNED = false;
                                CONTROLLABLE = true;
                                FRESH_CONTROL = true;
                                FINALIZED_STUN = true;
                            }
                        }
                        if StatusModule::status_kind(boss_boma) == *ITEM_LIOLEUSBOSS_STATUS_KIND_DOWN_AIR_END {
                            if MotionModule::frame(boss_boma) >= MotionModule::end_frame(boss_boma) - 2.0 {
                                STUNNED = false;
                                CONTROLLABLE = true;
                                FRESH_CONTROL = true;
                                FINALIZED_STUN = true;
                            }
                        }
                    }

                    if FighterInformation::is_operation_cpu(FighterManager::get_fighter_information(fighter_manager,smash::app::FighterEntryID(ENTRY_ID as i32))) == false && DEAD == false && !STUNNED {
                        if MotionModule::motion_kind(boss_boma) == smash::hash40("wait") {
                            CONTROLLABLE = true;
                        }
                        if MotionModule::motion_kind(boss_boma) == smash::hash40("hovering") {
                            CONTROLLABLE = true;
                        }
                        if StatusModule::status_kind(boss_boma) == *ITEM_LIOLEUSBOSS_STATUS_KIND_DOWN_START {
                            CONTROLLABLE = false;
                        }
                        if StatusModule::status_kind(boss_boma) == *ITEM_STATUS_KIND_NONE {
                            CONTROLLABLE = true;
                        }
                        if StatusModule::status_kind(boss_boma) == *ITEM_STATUS_KIND_WAIT {
                            CONTROLLABLE = true;
                        }
                        if StatusModule::status_kind(boss_boma) == *ITEM_LIOLEUSBOSS_STATUS_KIND_WAIT_AIR {
                            CONTROLLABLE = true;
                        }
                        if StatusModule::status_kind(boss_boma) == *ITEM_LIOLEUSBOSS_STATUS_KIND_WAIT {
                            CONTROLLABLE = true;
                        }
                        if StatusModule::status_kind(boss_boma) == *ITEM_LIOLEUSBOSS_STATUS_KIND_MOVE_TACKLE_TURN {
                            if MotionModule::frame(boss_boma) >= MotionModule::end_frame(boss_boma) - 2.0 {
                                CONTROLLABLE = true;
                                FRESH_CONTROL = true;
                            }
                        }
                        if StatusModule::status_kind(boss_boma) == *ITEM_STATUS_KIND_STANDBY {
                            CONTROLLABLE = true;
                            // StatusModule::change_status_request_from_script(boss_boma, *ITEM_STATUS_KIND_WAIT, true);
                        }
                        if StatusModule::status_kind(boss_boma) == *ITEM_LIOLEUSBOSS_STATUS_KIND_KIZETSU_END {
                            if MotionModule::frame(boss_boma) >= MotionModule::end_frame(boss_boma) - 2.0 {
                                CONTROLLABLE = true;
                                FRESH_CONTROL = true;
                            }
                        }
                        if StatusModule::status_kind(boss_boma) == *ITEM_LIOLEUSBOSS_STATUS_KIND_HOLE_END {
                            if MotionModule::frame(boss_boma) >= MotionModule::end_frame(boss_boma) - 2.0 {
                                CONTROLLABLE = true;
                                FRESH_CONTROL = true;
                            }
                        }
                        if StatusModule::status_kind(boss_boma) == *ITEM_LIOLEUSBOSS_STATUS_KIND_MOVE_TACKLE_END {
                            if MotionModule::frame(boss_boma) >= MotionModule::end_frame(boss_boma) - 2.0 {
                                CONTROLLABLE = true;
                                FRESH_CONTROL = true;
                            }
                        }
                        if StatusModule::status_kind(boss_boma) == *ITEM_LIOLEUSBOSS_STATUS_KIND_ATTACK_GLIDE_END {
                            if MotionModule::frame(boss_boma) >= MotionModule::end_frame(boss_boma) - 2.0 {
                                CONTROLLABLE = true;
                                FRESH_CONTROL = true;
                            }
                        }
                        if StatusModule::status_kind(boss_boma) == *ITEM_LIOLEUSBOSS_STATUS_KIND_ATTACK_FIREBALL_AIR {
                            if MotionModule::frame(boss_boma) >= MotionModule::end_frame(boss_boma) - 2.0 {
                                CONTROLLABLE = true;
                                FRESH_CONTROL = true;
                            }
                        }
                        if StatusModule::status_kind(boss_boma) == *ITEM_LIOLEUSBOSS_STATUS_KIND_ATTACK_GLIDE_END2 {
                            if MotionModule::frame(boss_boma) >= MotionModule::end_frame(boss_boma) - 2.0 {
                                CONTROLLABLE = true;
                                FRESH_CONTROL = true;
                            }
                        }
                        if StatusModule::status_kind(boss_boma) == *ITEM_LIOLEUSBOSS_STATUS_KIND_ATTACK_TACKLE_END {
                            if MotionModule::frame(boss_boma) >= MotionModule::end_frame(boss_boma) - 2.0 {
                                CONTROLLABLE = true;
                                FRESH_CONTROL = true;
                            }
                        }
                        if StatusModule::status_kind(boss_boma) == *ITEM_LIOLEUSBOSS_STATUS_KIND_DOWN_TAIL_CUT_END {
                            if MotionModule::frame(boss_boma) >= MotionModule::end_frame(boss_boma) - 2.0 {
                                CONTROLLABLE = true;
                                FRESH_CONTROL = true;
                            }
                        }
                        if StatusModule::status_kind(boss_boma) == *ITEM_LIOLEUSBOSS_STATUS_KIND_ATTACK_FIREBALL3_END {
                            if MotionModule::frame(boss_boma) >= MotionModule::end_frame(boss_boma) - 2.0 {
                                CONTROLLABLE = true;
                                FRESH_CONTROL = true;
                            }
                        }
                        if StatusModule::status_kind(boss_boma) == *ITEM_LIOLEUSBOSS_STATUS_KIND_ATTACK_FIREBALL3_AIR_END {
                            if MotionModule::frame(boss_boma) >= MotionModule::end_frame(boss_boma) - 2.0 {
                                CONTROLLABLE = true;
                                FRESH_CONTROL = true;
                            }
                        }
                        if StatusModule::status_kind(boss_boma) == *ITEM_LIOLEUSBOSS_STATUS_KIND_ATTACK_CHARGE_FIREBALL_END {
                            if MotionModule::frame(boss_boma) >= MotionModule::end_frame(boss_boma) - 2.0 {
                                // CONTROLLABLE = true;
                            }
                        }
                        if StatusModule::status_kind(boss_boma) == *ITEM_LIOLEUSBOSS_STATUS_KIND_FLY_END {
                            CONTROLLABLE = false;
                            if MotionModule::frame(boss_boma) >= MotionModule::end_frame(boss_boma) - 2.0 {
                                CONTROLLABLE = true;
                                FRESH_CONTROL = true;
                            }
                        }
                        if StatusModule::status_kind(boss_boma) == *ITEM_LIOLEUSBOSS_STATUS_KIND_ATTACK_GLIDE {
                            //Boss Control Stick Movement
                            if ControlModule::get_stick_x(module_accessor) <= 0.001 {
                                let pos = Vector3f{x: ControlModule::get_stick_x(module_accessor) * 2.0, y: 0.0, z: 0.0};
                                PostureModule::add_pos(boss_boma, &pos);
                            }
                        
                            if ControlModule::get_stick_x(module_accessor) >= -0.001 {
                                let pos = Vector3f{x: ControlModule::get_stick_x(module_accessor) * 2.0, y: 0.0, z: 0.0};
                                PostureModule::add_pos(boss_boma, &pos);
                            }
                        
                            if ControlModule::get_stick_y(module_accessor) <= 0.001 {
                                let pos = Vector3f{x: 0.0, y: ControlModule::get_stick_y(module_accessor) * 2.0, z: 0.0};
                                PostureModule::add_pos(boss_boma, &pos);
                            }
                        
                            if ControlModule::get_stick_y(module_accessor) >= -0.001 {
                                let pos = Vector3f{x: 0.0, y: ControlModule::get_stick_y(module_accessor) * 2.0, z: 0.0};
                                PostureModule::add_pos(boss_boma, &pos);
                            }
                        }
                        if StatusModule::status_kind(boss_boma) == *ITEM_LIOLEUSBOSS_STATUS_KIND_ATTACK_GLIDE_START {
                            //Boss Control Stick Movement
                            if ControlModule::get_stick_x(module_accessor) <= 0.001 {
                                let pos = Vector3f{x: ControlModule::get_stick_x(module_accessor) * 2.0, y: 0.0, z: 0.0};
                                PostureModule::add_pos(boss_boma, &pos);
                            }
                        
                            if ControlModule::get_stick_x(module_accessor) >= -0.001 {
                                let pos = Vector3f{x: ControlModule::get_stick_x(module_accessor) * 2.0, y: 0.0, z: 0.0};
                                PostureModule::add_pos(boss_boma, &pos);
                            }
                        
                            if ControlModule::get_stick_y(module_accessor) <= 0.001 {
                                let pos = Vector3f{x: 0.0, y: ControlModule::get_stick_y(module_accessor) * 2.0, z: 0.0};
                                PostureModule::add_pos(boss_boma, &pos);
                            }
                        
                            if ControlModule::get_stick_y(module_accessor) >= -0.001 {
                                let pos = Vector3f{x: 0.0, y: ControlModule::get_stick_y(module_accessor) * 2.0, z: 0.0};
                                PostureModule::add_pos(boss_boma, &pos);
                            }
                        }
                        if StatusModule::status_kind(boss_boma) == *ITEM_LIOLEUSBOSS_STATUS_KIND_FLY {
                            //Boss Control Stick Movement
                            if ControlModule::get_stick_x(module_accessor) <= 0.001 {
                                let pos = Vector3f{x: ControlModule::get_stick_x(module_accessor) * 2.0, y: 0.0, z: 0.0};
                                PostureModule::add_pos(boss_boma, &pos);
                            }
                        
                            if ControlModule::get_stick_x(module_accessor) >= -0.001 {
                                let pos = Vector3f{x: ControlModule::get_stick_x(module_accessor) * 2.0, y: 0.0, z: 0.0};
                                PostureModule::add_pos(boss_boma, &pos);
                            }
                        
                            if ControlModule::get_stick_y(module_accessor) <= 0.001 {
                                let pos = Vector3f{x: 0.0, y: ControlModule::get_stick_y(module_accessor) * 2.0, z: 0.0};
                                PostureModule::add_pos(boss_boma, &pos);
                            }
                        
                            if ControlModule::get_stick_y(module_accessor) >= -0.001 {
                                let pos = Vector3f{x: 0.0, y: ControlModule::get_stick_y(module_accessor) * 2.0, z: 0.0};
                                PostureModule::add_pos(boss_boma, &pos);
                            }
                        }
                        if StatusModule::status_kind(boss_boma) == *ITEM_LIOLEUSBOSS_STATUS_KIND_ATTACK_HOWLING_AIR {
                            //Boss Control Stick Movement
                            if ControlModule::get_stick_x(module_accessor) <= 0.001 {
                                let pos = Vector3f{x: ControlModule::get_stick_x(module_accessor) * 2.0, y: 0.0, z: 0.0};
                                PostureModule::add_pos(boss_boma, &pos);
                            }
                        
                            if ControlModule::get_stick_x(module_accessor) >= -0.001 {
                                let pos = Vector3f{x: ControlModule::get_stick_x(module_accessor) * 2.0, y: 0.0, z: 0.0};
                                PostureModule::add_pos(boss_boma, &pos);
                            }
                        
                            if ControlModule::get_stick_y(module_accessor) <= 0.001 {
                                let pos = Vector3f{x: 0.0, y: ControlModule::get_stick_y(module_accessor) * 2.0, z: 0.0};
                                PostureModule::add_pos(boss_boma, &pos);
                            }
                        
                            if ControlModule::get_stick_y(module_accessor) >= -0.001 {
                                let pos = Vector3f{x: 0.0, y: ControlModule::get_stick_y(module_accessor) * 2.0, z: 0.0};
                                PostureModule::add_pos(boss_boma, &pos);
                            }
                        }
                        if StatusModule::status_kind(boss_boma) == *ITEM_LIOLEUSBOSS_STATUS_KIND_ATTACK_NAIL_AIR {
                            //Boss Control Stick Movement
                            if ControlModule::get_stick_x(module_accessor) <= 0.001 {
                                let pos = Vector3f{x: ControlModule::get_stick_x(module_accessor) * 2.0, y: 0.0, z: 0.0};
                                PostureModule::add_pos(boss_boma, &pos);
                            }
                        
                            if ControlModule::get_stick_x(module_accessor) >= -0.001 {
                                let pos = Vector3f{x: ControlModule::get_stick_x(module_accessor) * 2.0, y: 0.0, z: 0.0};
                                PostureModule::add_pos(boss_boma, &pos);
                            }
                        
                            if ControlModule::get_stick_y(module_accessor) <= 0.001 {
                                let pos = Vector3f{x: 0.0, y: ControlModule::get_stick_y(module_accessor) * 2.0, z: 0.0};
                                PostureModule::add_pos(boss_boma, &pos);
                            }
                        
                            if ControlModule::get_stick_y(module_accessor) >= -0.001 {
                                let pos = Vector3f{x: 0.0, y: ControlModule::get_stick_y(module_accessor) * 2.0, z: 0.0};
                                PostureModule::add_pos(boss_boma, &pos);
                            }
                        }
                        if StatusModule::status_kind(boss_boma) == *ITEM_LIOLEUSBOSS_STATUS_KIND_STEP_AIR {
                            //Boss Control Stick Movement
                            if ControlModule::get_stick_x(module_accessor) <= 0.001 {
                                let pos = Vector3f{x: ControlModule::get_stick_x(module_accessor) * 2.0, y: 0.0, z: 0.0};
                                PostureModule::add_pos(boss_boma, &pos);
                            }
                        
                            if ControlModule::get_stick_x(module_accessor) >= -0.001 {
                                let pos = Vector3f{x: ControlModule::get_stick_x(module_accessor) * 2.0, y: 0.0, z: 0.0};
                                PostureModule::add_pos(boss_boma, &pos);
                            }
                        
                            if ControlModule::get_stick_y(module_accessor) <= 0.001 {
                                let pos = Vector3f{x: 0.0, y: ControlModule::get_stick_y(module_accessor) * 2.0, z: 0.0};
                                PostureModule::add_pos(boss_boma, &pos);
                            }
                        
                            if ControlModule::get_stick_y(module_accessor) >= -0.001 {
                                let pos = Vector3f{x: 0.0, y: ControlModule::get_stick_y(module_accessor) * 2.0, z: 0.0};
                                PostureModule::add_pos(boss_boma, &pos);
                            }
                        }
                        if StatusModule::status_kind(boss_boma) == *ITEM_LIOLEUSBOSS_STATUS_KIND_TURN_AIR {
                            if MotionModule::frame(boss_boma) >= MotionModule::end_frame(boss_boma) - 2.0 {
                                CONTROLLABLE = true;
                                FRESH_CONTROL = true;
                            }
                        }
                        if StatusModule::status_kind(boss_boma) == *ITEM_LIOLEUSBOSS_STATUS_KIND_WAIT_AIR {
                            //Boss Control Stick Movement
                            if ControlModule::get_stick_x(module_accessor) <= 0.001 {
                                let pos = Vector3f{x: ControlModule::get_stick_x(module_accessor) * 2.0, y: 0.0, z: 0.0};
                                PostureModule::add_pos(boss_boma, &pos);
                            }
                        
                            if ControlModule::get_stick_x(module_accessor) >= -0.001 {
                                let pos = Vector3f{x: ControlModule::get_stick_x(module_accessor) * 2.0, y: 0.0, z: 0.0};
                                PostureModule::add_pos(boss_boma, &pos);
                            }
                        
                            if ControlModule::get_stick_y(module_accessor) <= 0.001 {
                                let pos = Vector3f{x: 0.0, y: ControlModule::get_stick_y(module_accessor) * 2.0, z: 0.0};
                                PostureModule::add_pos(boss_boma, &pos);
                            }
                        
                            if ControlModule::get_stick_y(module_accessor) >= -0.001 {
                                let pos = Vector3f{x: 0.0, y: ControlModule::get_stick_y(module_accessor) * 2.0, z: 0.0};
                                PostureModule::add_pos(boss_boma, &pos);
                            }
                        }
                        if StatusModule::status_kind(boss_boma) == *ITEM_LIOLEUSBOSS_STATUS_KIND_DOWN_AIR_START {
                            CONTROLLABLE = false;
                        }
                        if StatusModule::status_kind(boss_boma) == *ITEM_LIOLEUSBOSS_STATUS_KIND_DOWN_AIR_LOOP {
                            CONTROLLABLE = false;
                        }
                        if StatusModule::status_kind(boss_boma) == *ITEM_STATUS_KIND_TURN {
                            if MotionModule::frame(boss_boma) >= MotionModule::end_frame(boss_boma) - 2.0 {
                                CONTROLLABLE = true;
                                FRESH_CONTROL = true;
                            }
                        }
                        if StatusModule::status_kind(boss_boma) == *ITEM_LIOLEUSBOSS_STATUS_KIND_ATTACK_BACK_JUMP_FIREBALL {
                            if MotionModule::frame(boss_boma) >= MotionModule::end_frame(boss_boma) - 2.0 {
                                CONTROLLABLE = true;
                                FRESH_CONTROL = true;
                            }
                        }
                        if StatusModule::status_kind(boss_boma) == *ITEM_LIOLEUSBOSS_STATUS_KIND_ATTACK_BACK_JUMP_FIREBALL2 {
                            if MotionModule::frame(boss_boma) >= MotionModule::end_frame(boss_boma) - 2.0 {
                                CONTROLLABLE = true;
                                FRESH_CONTROL = true;
                            }
                        }
                        if StatusModule::status_kind(boss_boma) == *ITEM_LIOLEUSBOSS_STATUS_KIND_TURN {
                            if MotionModule::frame(boss_boma) >= MotionModule::end_frame(boss_boma) - 2.0 {
                                CONTROLLABLE = true;
                                FRESH_CONTROL = true;
                            }
                        }
                        if StatusModule::status_kind(boss_boma) == *ITEM_LIOLEUSBOSS_STATUS_KIND_ATTACK_HOWLING {
                            if MotionModule::frame(boss_boma) >= MotionModule::end_frame(boss_boma) - 2.0 {
                                CONTROLLABLE = true;
                                FRESH_CONTROL = true;
                            }
                        }
                        if StatusModule::status_kind(boss_boma) == *ITEM_LIOLEUSBOSS_STATUS_KIND_ATTACK_HOWLING_AIR {
                            if MotionModule::frame(boss_boma) >= MotionModule::end_frame(boss_boma) - 2.0 {
                                CONTROLLABLE = true;
                                FRESH_CONTROL = true;
                            }
                        }
                        if StatusModule::status_kind(boss_boma) == *ITEM_LIOLEUSBOSS_STATUS_KIND_ATTACK_NAIL_AIR {
                            if MotionModule::frame(boss_boma) >= MotionModule::end_frame(boss_boma) - 2.0 {
                                CONTROLLABLE = true;
                                FRESH_CONTROL = true;
                            }
                        }
                        if StatusModule::status_kind(boss_boma) == *ITEM_LIOLEUSBOSS_STATUS_KIND_CHANGE_MODE_AIR {
                            if MotionModule::frame(boss_boma) >= MotionModule::end_frame(boss_boma) - 2.0 {
                                CONTROLLABLE = true;
                                FRESH_CONTROL = true;
                            }
                        }
                        if StatusModule::status_kind(boss_boma) == *ITEM_LIOLEUSBOSS_STATUS_KIND_CHANGE_MODE_GROUND {
                            if MotionModule::frame(boss_boma) >= MotionModule::end_frame(boss_boma) - 2.0 {
                                CONTROLLABLE = true;
                                FRESH_CONTROL = true;
                            }
                        }

                        if CONTROLLABLE == false {
                            if GROUNDED == true {
                                if DEAD == false {
                                    if StatusModule::status_kind(boss_boma) == *ITEM_LIOLEUSBOSS_STATUS_KIND_ATTACK_TACKLE {
                                        if lua_bind::PostureModule::lr(boss_boma) == 1.0 { // right
                                            if ControlModule::get_stick_x(module_accessor) < -0.1 {
                                                StatusModule::change_status_request_from_script(boss_boma, *ITEM_LIOLEUSBOSS_STATUS_KIND_ATTACK_TACKLE_TURN, true);
                                            }
                                        }
                                        if lua_bind::PostureModule::lr(boss_boma) == -1.0 { // left
                                            if ControlModule::get_stick_x(module_accessor) > 0.0 {
                                                StatusModule::change_status_request_from_script(boss_boma, *ITEM_LIOLEUSBOSS_STATUS_KIND_ATTACK_TACKLE_TURN, true);
                                            }
                                        }
                                    }
                                }
                            }
                        }

                        if CONTROLLABLE == true && GROUNDED == true && DEAD == false && sv_information::is_ready_go() == true {
                            //Boss Moves
                            if ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_JUMP) {
                                CONTROLLABLE = false;
                                GROUNDED = false;
                                StatusModule::change_status_request_from_script(boss_boma, *ITEM_LIOLEUSBOSS_STATUS_KIND_CHANGE_MODE_AIR, true);
                            }
                            if ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
                                CONTROLLABLE = false;
                                StatusModule::change_status_request_from_script(boss_boma, *ITEM_LIOLEUSBOSS_STATUS_KIND_ATTACK_BACK_JUMP_FIREBALL, true);
                            }
                            if ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_GUARD) {
                                CONTROLLABLE = false;
                                StatusModule::change_status_request_from_script(boss_boma, *ITEM_LIOLEUSBOSS_STATUS_KIND_TURN, true);
                            }
                            if ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
                                CONTROLLABLE = false;
                                StatusModule::change_status_request_from_script(boss_boma, *ITEM_LIOLEUSBOSS_STATUS_KIND_ATTACK_TACKLE_START, true);
                            }
                            if ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW != 0 {
                                CONTROLLABLE = false;
                                StatusModule::change_status_request_from_script(boss_boma, *ITEM_LIOLEUSBOSS_STATUS_KIND_ATTACK_HOWLING, true);
                            }
                            if ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_HI != 0 {
                                CONTROLLABLE = false;
                                StatusModule::change_status_request_from_script(boss_boma, *ITEM_LIOLEUSBOSS_STATUS_KIND_ATTACK_ASSAULT_START, true);
                            }
                            if ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_S != 0 {
                                CONTROLLABLE = false;
                                StatusModule::change_status_request_from_script(boss_boma, *ITEM_LIOLEUSBOSS_STATUS_KIND_ATTACK_BACK_JUMP_FIREBALL2, true);
                            }
                            if ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW3 != 0 {
                                CONTROLLABLE = false;
                                StatusModule::change_status_request_from_script(boss_boma, *ITEM_LIOLEUSBOSS_STATUS_KIND_ATTACK_FIREBALL, true);
                            }
                            if ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI3 != 0 {
                                CONTROLLABLE = false;
                                StatusModule::change_status_request_from_script(boss_boma, *ITEM_LIOLEUSBOSS_STATUS_KIND_ATTACK_FIREBALL3_START, true);
                            }
                            if ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S3 != 0 {
                                CONTROLLABLE = false;
                                StatusModule::change_status_request_from_script(boss_boma, *ITEM_LIOLEUSBOSS_STATUS_KIND_ATTACK_TACKLE_START, true);
                            }
                            if ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_APPEAL_HI) {
                                CONTROLLABLE = false;
                                StatusModule::change_status_request_from_script(boss_boma, *ITEM_LIOLEUSBOSS_STATUS_KIND_ATTACK_CHARGE_FIREBALL_START, true);
                            }
                        }

                        if CONTROLLABLE == true && GROUNDED == false && DEAD == false && sv_information::is_ready_go() == true {
                            //Boss Control Stick Movement
                            if ControlModule::get_stick_x(module_accessor) <= 0.001 {
                                let pos = Vector3f{x: ControlModule::get_stick_x(module_accessor) * 2.0, y: 0.0, z: 0.0};
                                PostureModule::add_pos(boss_boma, &pos);
                            }
                        
                            if ControlModule::get_stick_x(module_accessor) >= -0.001 {
                                let pos = Vector3f{x: ControlModule::get_stick_x(module_accessor) * 2.0, y: 0.0, z: 0.0};
                                PostureModule::add_pos(boss_boma, &pos);
                            }
                        
                            if ControlModule::get_stick_y(module_accessor) <= 0.001 {
                                let pos = Vector3f{x: 0.0, y: ControlModule::get_stick_y(module_accessor) * 2.0, z: 0.0};
                                PostureModule::add_pos(boss_boma, &pos);
                            }
                        
                            if ControlModule::get_stick_y(module_accessor) >= -0.001 {
                                let pos = Vector3f{x: 0.0, y: ControlModule::get_stick_y(module_accessor) * 2.0, z: 0.0};
                                PostureModule::add_pos(boss_boma, &pos);
                            }
                            //Boss Moves
                            if ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_JUMP) {
                                CONTROLLABLE = false;
                                GROUNDED = true;
                                StatusModule::change_status_request_from_script(boss_boma, *ITEM_LIOLEUSBOSS_STATUS_KIND_CHANGE_MODE_GROUND, true);
                            }
                            if ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
                                CONTROLLABLE = false;
                                StatusModule::change_status_request_from_script(boss_boma, *ITEM_LIOLEUSBOSS_STATUS_KIND_ATTACK_CHARGE_FIREBALL_START_AIR, true);
                            }
                            if ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_GUARD) {
                                CONTROLLABLE = false;
                                StatusModule::change_status_request_from_script(boss_boma, *ITEM_LIOLEUSBOSS_STATUS_KIND_TURN_AIR, true);
                            }
                            if ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
                                CONTROLLABLE = false;
                                StatusModule::change_status_request_from_script(boss_boma, *ITEM_LIOLEUSBOSS_STATUS_KIND_ATTACK_FIREBALL_AIR, true);
                            }
                            if ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW != 0 {
                                CONTROLLABLE = false;
                                StatusModule::change_status_request_from_script(boss_boma, *ITEM_LIOLEUSBOSS_STATUS_KIND_ATTACK_GLIDE_START, true);
                            }
                            if ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_HI != 0 {
                                CONTROLLABLE = false;
                                StatusModule::change_status_request_from_script(boss_boma, *ITEM_LIOLEUSBOSS_STATUS_KIND_ATTACK_HOWLING_AIR, true);
                            }
                            if ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_S != 0 {
                                CONTROLLABLE = false;
                                StatusModule::change_status_request_from_script(boss_boma, *ITEM_LIOLEUSBOSS_STATUS_KIND_ATTACK_NAIL_AIR, true);
                            }
                            if ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW3 != 0 {
                                CONTROLLABLE = false;
                                StatusModule::change_status_request_from_script(boss_boma, *ITEM_LIOLEUSBOSS_STATUS_KIND_ATTACK_HOWLING_AIR, true);
                            }
                            if ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI3 != 0 {
                                CONTROLLABLE = false;
                                StatusModule::change_status_request_from_script(boss_boma, *ITEM_LIOLEUSBOSS_STATUS_KIND_ATTACK_FIREBALL3_AIR_START, true);
                            }
                            if ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S3 != 0 {
                                CONTROLLABLE = false;
                                StatusModule::change_status_request_from_script(boss_boma, *ITEM_LIOLEUSBOSS_STATUS_KIND_ATTACK_FIREBALL3_AIR_START, true);
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
