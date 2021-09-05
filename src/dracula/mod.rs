use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash::lua2cpp::L2CFighterCommon;
use smash::app::BattleObjectModuleAccessor;
use smash::phx::Vector3f;
use smash_script::macros::WHOLE_HIT;
use smash::app::ItemKind;
use smash::app::sv_battle_object;
use std::u32;
use smash::app::sv_information;
use skyline::nn::ro::LookupSymbol;

static mut SPAWN_BOSS : bool = true;
static mut HAVE_ITEM : bool = false;
static mut ENTRANCE_ANIM : bool = false;
static mut GAME_START : bool = false;
static mut STOP_CONTROL_LOOP : bool = false;
static mut ENTRY_ID : usize = 0;
static mut BOSS_ID : [u32; 8] = [0; 8];
static mut IS_BOSS_DEAD : bool = false;
pub static mut FIGHTER_MANAGER: usize = 0;
static mut TRANSFORMED_MODE : bool = false;
static mut KICKSTART_ANIM_BEGIN : bool = false;
static mut TRANSITIONING : bool = false;

pub fn once_per_fighter_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
            let lua_state = fighter.lua_state_agent;
            let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
            let fighter_kind = smash::app::utility::get_kind(module_accessor);
            pub unsafe fn entry_id(module_accessor: &mut BattleObjectModuleAccessor) -> usize {
                let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
                return entry_id;
            }
            ENTRY_ID = WorkModule::get_int(module_accessor,*FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
            LookupSymbol(
                &mut FIGHTER_MANAGER,
                "_ZN3lib9SingletonIN3app14FighterManagerEE9instance_E\u{0}"
                .as_bytes()
                .as_ptr(),
            );
            if fighter_kind == *FIGHTER_KIND_CHROM {
                if HAVE_ITEM == true {
                    if sv_information::is_ready_go() == true {
                        let boss_boma = sv_battle_object::module_accessor(BOSS_ID[entry_id(module_accessor)]);
                        StatusModule::change_status_request_from_script(module_accessor,*FIGHTER_STATUS_KIND_FALL_SPECIAL,true);
                        let x = PostureModule::pos_x(boss_boma);
                        let y = PostureModule::pos_y(boss_boma);
                        let z = PostureModule::pos_z(boss_boma);
                        let boss_pos = Vector3f{x: x, y: y + 30.0, z: z};
                        if PostureModule::pos_y(boss_boma) >= 145.0 {
                            let boss_y_pos = Vector3f{x: x, y: 145.0, z: z};
                            PostureModule::set_pos(module_accessor, &boss_y_pos);
                        }
                        else if PostureModule::pos_y(boss_boma) <= -100.0 {
                            let boss_y_pos = Vector3f{x: x, y: -100.0, z: z};
                            PostureModule::set_pos(module_accessor, &boss_y_pos);
                        }
                        else if PostureModule::pos_x(boss_boma) >= 175.0 {
                            let boss_x_pos = Vector3f{x: 175.0, y: y, z: z};
                            PostureModule::set_pos(module_accessor, &boss_x_pos);
                        }
                        else if PostureModule::pos_x(boss_boma) <= -175.0 {
                            let boss_x_pos = Vector3f{x: -175.0, y: y, z: z};
                            PostureModule::set_pos(module_accessor, &boss_x_pos);
                        }
                        else {
                            PostureModule::set_pos(module_accessor, &boss_pos);
                        }
                    }
                }
            }
            let fighter_manager = *(FIGHTER_MANAGER as *mut *mut smash::app::FighterManager);
        //if smash::lib::lua_const::FIGHTER_INSTANCE_WORK_ID_INT_COLOR == 0 {
            if FighterInformation::is_operation_cpu(FighterManager::get_fighter_information(fighter_manager,smash::app::FighterEntryID(ENTRY_ID as i32))) == true {
                let boss_boma = sv_battle_object::module_accessor(BOSS_ID[entry_id(module_accessor)]);
                if fighter_kind == *FIGHTER_KIND_CHROM {
                    if MotionModule::frame(fighter.module_accessor) >= 29.0 {
                        if sv_information::is_ready_go() == false {
                            TRANSITIONING = false;
                            TRANSFORMED_MODE = false;
                            HAVE_ITEM = false;
                            IS_BOSS_DEAD = false;
                            let lua_state = fighter.lua_state_agent;
                            let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
                            ENTRY_ID = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
                            if SPAWN_BOSS == true {
                                let boss_boma = sv_battle_object::module_accessor(BOSS_ID[entry_id(module_accessor)]);
                                
                                ENTRANCE_ANIM = false;
                                ItemModule::have_item(module_accessor,ItemKind(*ITEM_KIND_DRACULA),0,0,false,false);
                                    BOSS_ID[entry_id(module_accessor)] = ItemModule::get_have_item_id(module_accessor,0) as u32;
                                ModelModule::set_scale(module_accessor,0.0001);
                                HAVE_ITEM = true;
                                ItemModule::throw_item(fighter.module_accessor, 0.0, 0.0, 0.0, 0, true, 0.0);
                                StatusModule::change_status_request_from_script(module_accessor,*FIGHTER_STATUS_KIND_STANDBY,true);
                                StatusModule::change_status_request_from_script(boss_boma, *ITEM_DRACULA_STATUS_KIND_WAIT, true);
                                StatusModule::change_status_request_from_script(module_accessor,*FIGHTER_STATUS_KIND_STANDBY,true);
                            }
                        }
                    }

                    DamageModule::set_damage_lock(boss_boma, true);
                    WHOLE_HIT(fighter, *HIT_STATUS_XLU);
                    HitModule::set_whole(module_accessor, smash::app::HitStatus(*HIT_STATUS_XLU), 0);


                    if StopModule::is_damage(boss_boma) {
                        if TRANSFORMED_MODE == false {
                            if DamageModule::damage(module_accessor, 0) >= 199.0 {
                                let x = PostureModule::pos_x(boss_boma);
                                let z = PostureModule::pos_z(boss_boma);
                                let none_pos = Vector3f{x: x, y: 0.0, z: z};

                                if StatusModule::status_kind(boss_boma) != *ITEM_STATUS_KIND_DEAD {
                                    StatusModule::change_status_request_from_script(boss_boma,*ITEM_STATUS_KIND_DEAD,true);
                                    TRANSITIONING = true;
                                }

                                if TRANSITIONING == true {
                                    if StatusModule::status_kind(boss_boma) != *ITEM_STATUS_KIND_DEAD {
                                        if StatusModule::status_kind(boss_boma) != *ITEM_DRACULA_STATUS_KIND_CHANGE_START {
                                            TRANSFORMED_MODE = true;
                                            ENTRANCE_ANIM = false;
                                            PostureModule::set_pos(module_accessor, &none_pos);
                                            PostureModule::set_pos(boss_boma, &none_pos);
                                            StatusModule::change_status_request_from_script(boss_boma,*ITEM_STATUS_KIND_STANDBY,true);
                                            let boss_boma = sv_battle_object::module_accessor(BOSS_ID[entry_id(module_accessor)]);
                                            ItemModule::have_item(module_accessor,ItemKind(*ITEM_KIND_DRACULA2),0,0,false,false);
                                                BOSS_ID[entry_id(module_accessor)] = ItemModule::get_have_item_id(module_accessor,0) as u32;
                                            PostureModule::set_pos(module_accessor, &none_pos);
                                            PostureModule::set_pos(boss_boma, &none_pos);
                                            TRANSITIONING = false;
                                        }
                                    }
                                }
                            }
                        }
                    }

                    if StopModule::is_damage(boss_boma) {
                        if DamageModule::damage(module_accessor, 0) >= 399.0 {
                            if IS_BOSS_DEAD == false {
                                IS_BOSS_DEAD = true;
                                StatusModule::change_status_request_from_script(boss_boma,*ITEM_STATUS_KIND_DEAD,true);
                                StatusModule::change_status_request_from_script(module_accessor,*FIGHTER_STATUS_KIND_DEAD,true);
                            }
                        }
                        if DamageModule::damage(module_accessor, 0) == -1.0 {
                            if IS_BOSS_DEAD == false {
                                IS_BOSS_DEAD = true;
                                StatusModule::change_status_request_from_script(boss_boma,*ITEM_STATUS_KIND_DEAD,true);
                                StatusModule::change_status_request_from_script(module_accessor,*FIGHTER_STATUS_KIND_DEAD,true);
                            }
                        }
                        DamageModule::add_damage(module_accessor, 4.1, 0);
                        if StopModule::is_stop(module_accessor) {
                            StopModule::end_stop(module_accessor);
                        }
                        if StopModule::is_stop(boss_boma) {
                            StopModule::end_stop(boss_boma);
                        }
                    }

                    if StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_DEAD {
                        if IS_BOSS_DEAD == false {
                            IS_BOSS_DEAD = true;
                            StatusModule::change_status_request_from_script(boss_boma,*ITEM_STATUS_KIND_DEAD,true);
                        }
                    }

                    if IS_BOSS_DEAD == true {
                        if sv_information::is_ready_go() == true {
                            if FighterInformation::stock_count(FighterManager::get_fighter_information(fighter_manager,smash::app::FighterEntryID(ENTRY_ID as i32))) != 0 {
                                StatusModule::change_status_request_from_script(module_accessor,*FIGHTER_STATUS_KIND_DEAD,true);
                                DamageModule::add_damage(module_accessor, 400.0, 0);
                                STOP_CONTROL_LOOP = false;
                            }
                        }
                    }

                    if MotionModule::frame(fighter.module_accessor) >= 30.0 {
                        if sv_information::is_ready_go() == true {
                            HAVE_ITEM = true;
                        }
                    }

                    if sv_information::is_ready_go() == true {
                        if HAVE_ITEM == true {
                            if IS_BOSS_DEAD == false {
                                MotionModule::set_rate(boss_boma, 1.0);
                            }
                        }
                    }

                    if sv_information::is_ready_go() == false {
                        if HAVE_ITEM == true {
                            if IS_BOSS_DEAD == false {
                                StatusModule::change_status_request_from_script(boss_boma,*ITEM_DRACULA_STATUS_KIND_WAIT,true);
                                MotionModule::set_rate(boss_boma, 0.0);
                            }
                        }
                    }
                }
            }
            else {
                let boss_boma = sv_battle_object::module_accessor(BOSS_ID[entry_id(module_accessor)]);
                if SPAWN_BOSS == true {
                    if fighter_kind == *FIGHTER_KIND_CHROM {
                        if MotionModule::frame(fighter.module_accessor) >= 29.0 {
                            if sv_information::is_ready_go() == false {
                                KICKSTART_ANIM_BEGIN = false;
                                TRANSITIONING = false;
                                TRANSFORMED_MODE = false;
                                GAME_START = false;
                                HAVE_ITEM = false;
                                IS_BOSS_DEAD = false;
                                STOP_CONTROL_LOOP = true;
                                let lua_state = fighter.lua_state_agent;
                                let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
                                ENTRY_ID = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
                                if SPAWN_BOSS == true {
                                    
                                    ENTRANCE_ANIM = false;
                                    ItemModule::have_item(module_accessor,ItemKind(*ITEM_KIND_DRACULA),0,0,false,false);
                                        BOSS_ID[entry_id(module_accessor)] = ItemModule::get_have_item_id(module_accessor,0) as u32;
                                    ModelModule::set_scale(module_accessor,0.0001);
                                    HAVE_ITEM = true;
                                }
                            }
                        }

                        DamageModule::set_damage_lock(boss_boma,true);
                        WHOLE_HIT(fighter, *HIT_STATUS_XLU);
                        HitModule::set_whole(module_accessor, smash::app::HitStatus(*HIT_STATUS_XLU), 0);

                        if StopModule::is_damage(boss_boma) {
                            if TRANSFORMED_MODE == false {
                                if DamageModule::damage(module_accessor, 0) >= 199.0 {
                                    let x = PostureModule::pos_x(boss_boma);
                                    let z = PostureModule::pos_z(boss_boma);
                                    let none_pos = Vector3f{x: x, y: 0.0, z: z};
    
                                    if StatusModule::status_kind(boss_boma) != *ITEM_STATUS_KIND_DEAD {
                                        StatusModule::change_status_request_from_script(boss_boma,*ITEM_STATUS_KIND_DEAD,true);
                                        TRANSITIONING = true;
                                    }
    
                                    if TRANSITIONING == true {
                                        if StatusModule::status_kind(boss_boma) != *ITEM_STATUS_KIND_DEAD {
                                            if StatusModule::status_kind(boss_boma) != *ITEM_DRACULA_STATUS_KIND_CHANGE_START {
                                                TRANSFORMED_MODE = true;
                                                ENTRANCE_ANIM = false;
                                                PostureModule::set_pos(module_accessor, &none_pos);
                                                PostureModule::set_pos(boss_boma, &none_pos);
                                                StatusModule::change_status_request_from_script(boss_boma,*ITEM_STATUS_KIND_STANDBY,true);
                                                let boss_boma = sv_battle_object::module_accessor(BOSS_ID[entry_id(module_accessor)]);
                                                ItemModule::have_item(module_accessor,ItemKind(*ITEM_KIND_DRACULA2),0,0,false,false);
                                                    BOSS_ID[entry_id(module_accessor)] = ItemModule::get_have_item_id(module_accessor,0) as u32;
                                                PostureModule::set_pos(module_accessor, &none_pos);
                                                PostureModule::set_pos(boss_boma, &none_pos);
                                                TRANSITIONING = false;
                                            }
                                        }
                                    }
                                }
                            }
                        }

                        if STOP_CONTROL_LOOP == true {
                            MotionModule::set_rate(boss_boma, 1.0);
                        }

                        if sv_information::is_ready_go() == false {
                            GAME_START = false;
                        }

                        if StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_DEAD {
                            if IS_BOSS_DEAD == false {
                                IS_BOSS_DEAD = true;
                                StatusModule::change_status_request_from_script(boss_boma,*ITEM_STATUS_KIND_DEAD,true);
                            }
                        }

                        if sv_information::is_ready_go() == true {
                            GAME_START = true;
                        }

                        if IS_BOSS_DEAD == true {
                            if sv_information::is_ready_go() == true {
                                if FighterInformation::stock_count(FighterManager::get_fighter_information(fighter_manager,smash::app::FighterEntryID(ENTRY_ID as i32))) != 0 {
                                    StatusModule::change_status_request_from_script(module_accessor,*FIGHTER_STATUS_KIND_DEAD,true);
                                    DamageModule::add_damage(module_accessor, 400.0, 0);
                                    STOP_CONTROL_LOOP = false;
                                }
                            }
                        }

                        if FighterInformation::stock_count(FighterManager::get_fighter_information(fighter_manager,smash::app::FighterEntryID(ENTRY_ID as i32))) == 0 {
                            if StatusModule::status_kind(module_accessor) != *FIGHTER_STATUS_KIND_STANDBY {
                                StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_STANDBY,true);
                            }
                        }

                        if StopModule::is_damage(boss_boma) {
                            if DamageModule::damage(module_accessor, 0) >= 399.0 {
                                if IS_BOSS_DEAD == false {
                                    IS_BOSS_DEAD = true;
                                    StatusModule::change_status_request_from_script(boss_boma,*ITEM_STATUS_KIND_DEAD,true);
                                    StatusModule::change_status_request_from_script(module_accessor,*FIGHTER_STATUS_KIND_DEAD,true);
                                }
                            }
                            if DamageModule::damage(module_accessor, 0) == -1.0 {
                                if IS_BOSS_DEAD == false {
                                    IS_BOSS_DEAD = true;
                                    StatusModule::change_status_request_from_script(boss_boma,*ITEM_STATUS_KIND_DEAD,true);
                                    StatusModule::change_status_request_from_script(module_accessor,*FIGHTER_STATUS_KIND_DEAD,true);
                                }
                            }
                            DamageModule::add_damage(module_accessor, 4.1, 0);
                            if StopModule::is_stop(module_accessor) {
                                StopModule::end_stop(module_accessor);
                            }
                            if StopModule::is_stop(boss_boma) {
                                StopModule::end_stop(boss_boma);
                            }
                        }

                        if sv_information::is_ready_go() == false {
                            STOP_CONTROL_LOOP = true;
                        }

                        if MotionModule::frame(fighter.module_accessor) >= 30.0 {
                            if sv_information::is_ready_go() == true {
                                HAVE_ITEM = true;
                            }
                        }

                        if GAME_START == true {
                            if STOP_CONTROL_LOOP == true {
                                if HAVE_ITEM == true {
                                    if IS_BOSS_DEAD == false {
                                        if StatusModule::status_kind(boss_boma) == *ITEM_STATUS_KIND_WAIT {
                                            
                                        }
                                        else if StatusModule::status_kind(boss_boma) == *ITEM_STATUS_KIND_ENTRY {

                                        }
                                        else if StatusModule::status_kind(boss_boma) == *ITEM_STATUS_KIND_RUN {

                                        }
                                        else if StatusModule::status_kind(boss_boma) == *ITEM_STATUS_KIND_TURN {

                                        }
                                        else if StatusModule::status_kind(boss_boma) == *ITEM_STATUS_KIND_NONE {

                                        }
                                        else if StatusModule::status_kind(boss_boma) == *ITEM_STATUS_KIND_JUMP {

                                        }
                                        else if StatusModule::status_kind(boss_boma) == *ITEM_STATUS_KIND_JUMP_AIR {

                                        }
                                        else if StatusModule::status_kind(boss_boma) == *ITEM_STATUS_KIND_FALL {

                                        }
                                        else if StatusModule::status_kind(boss_boma) == *ITEM_STATUS_KIND_EXIT {

                                        }
                                        else if StatusModule::status_kind(boss_boma) == *ITEM_STATUS_KIND_PASS_FLOOR {

                                        }
                                        else if StatusModule::status_kind(boss_boma) == *ITEM_STATUS_KIND_START {
                                            
                                        }
                                        else if StatusModule::status_kind(boss_boma) == *ITEM_STATUS_KIND_TERM {
                                            
                                        }
                                        else if StatusModule::status_kind(boss_boma) == *ITEM_STATUS_KIND_LANDING {
                                            
                                        }
                                        else if StatusModule::status_kind(boss_boma) == *ITEM_STATUS_KIND_INITIALIZE {

                                        }
                                        else {
                                            if TRANSFORMED_MODE == false {
                                                if StatusModule::status_kind(boss_boma) == *ITEM_STATUS_KIND_ENTRY {

                                                }
                                                else {
                                                    StatusModule::change_status_request_from_script(boss_boma, *ITEM_STATUS_KIND_WAIT, true);
                                                }
                                            }
                                            if TRANSFORMED_MODE == true {
                                                if StatusModule::status_kind(boss_boma) == *ITEM_STATUS_KIND_ENTRY {

                                                }
                                                else {
                                                    StatusModule::change_status_request_from_script(boss_boma, *ITEM_DRACULA2_STATUS_KIND_WAIT, true);
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }

                        if HAVE_ITEM == true {
                            if StatusModule::status_kind(boss_boma) == *ITEM_STATUS_KIND_WAIT {
                                STOP_CONTROL_LOOP = true;
                            }

                            if StatusModule::status_kind(boss_boma) == *ITEM_DRACULA_STATUS_KIND_WAIT {
                                STOP_CONTROL_LOOP = true;
                            }

                            if StatusModule::status_kind(boss_boma) == *ITEM_DRACULA2_STATUS_KIND_WAIT {
                                STOP_CONTROL_LOOP = true;
                            }

                            if TRANSFORMED_MODE == false {
                                if StatusModule::status_kind(boss_boma) == *ITEM_STATUS_KIND_STANDBY {
                                    STOP_CONTROL_LOOP = true;
                                    StatusModule::change_status_request_from_script(boss_boma, *ITEM_DRACULA_STATUS_KIND_ATTACK_STRAIGHT_END, true);
                                }
                            }

                            if StatusModule::status_kind(boss_boma) == *ITEM_DRACULA_STATUS_KIND_TELEPORT_END {
                                if MotionModule::frame(boss_boma) == MotionModule::end_frame(boss_boma) {
                                    STOP_CONTROL_LOOP = true;
                                }
                            }

                            if StatusModule::status_kind(boss_boma) == *ITEM_DRACULA_STATUS_KIND_ATTACK_3WAY_END {
                                if MotionModule::frame(boss_boma) == MotionModule::end_frame(boss_boma) {
                                    STOP_CONTROL_LOOP = true;
                                }
                            }

                            if StatusModule::status_kind(boss_boma) == *ITEM_DRACULA_STATUS_KIND_ATTACK_FILL_END {
                                if MotionModule::frame(boss_boma) == MotionModule::end_frame(boss_boma) {
                                    STOP_CONTROL_LOOP = true;
                                }
                            }

                            if StatusModule::status_kind(boss_boma) == *ITEM_DRACULA_STATUS_KIND_ATTACK_RUSH_END {
                                if MotionModule::frame(boss_boma) == MotionModule::end_frame(boss_boma) {
                                    STOP_CONTROL_LOOP = true;
                                }
                            }

                            if StatusModule::status_kind(boss_boma) == *ITEM_DRACULA_STATUS_KIND_ATTACK_PILLAR_END {
                                STOP_CONTROL_LOOP = true;
                                StatusModule::change_status_request_from_script(boss_boma, *ITEM_STATUS_KIND_WAIT, true);
                            }

                            if StatusModule::status_kind(boss_boma) == *ITEM_DRACULA_STATUS_KIND_ATTACK_STRAIGHT_END {
                                if MotionModule::frame(boss_boma) == MotionModule::end_frame(boss_boma) {
                                    STOP_CONTROL_LOOP = true;
                                }
                            }

                            if StatusModule::status_kind(boss_boma) == *ITEM_DRACULA_STATUS_KIND_ATTACK_TURN_3WAY_END {
                                if MotionModule::frame(boss_boma) == MotionModule::end_frame(boss_boma) {
                                    STOP_CONTROL_LOOP = true;
                                }
                            }

                            if StatusModule::status_kind(boss_boma) == *ITEM_DRACULA2_STATUS_KIND_SQUASH_END {
                                if MotionModule::frame(boss_boma) == MotionModule::end_frame(boss_boma) {
                                    STOP_CONTROL_LOOP = true;
                                }
                            }

                            if StatusModule::status_kind(boss_boma) == *ITEM_DRACULA2_STATUS_KIND_FIRE_SHOT_END {
                                if MotionModule::frame(boss_boma) == MotionModule::end_frame(boss_boma) {
                                    STOP_CONTROL_LOOP = true;
                                }
                            }
                            if StatusModule::status_kind(boss_boma) == *ITEM_DRACULA2_STATUS_KIND_HOMING_SHOT_END {
                                if MotionModule::frame(boss_boma) == MotionModule::end_frame(boss_boma) {
                                    STOP_CONTROL_LOOP = true;
                                }
                            }
                            if StatusModule::status_kind(boss_boma) == *ITEM_DRACULA2_STATUS_KIND_SQUASH_END_TURN {
                                if MotionModule::frame(boss_boma) == MotionModule::end_frame(boss_boma) {
                                    STOP_CONTROL_LOOP = true;
                                }
                            }
                        }

                        if STOP_CONTROL_LOOP == true {
                            if TRANSFORMED_MODE == false {
                                if HAVE_ITEM  == true {
                                    if sv_information::is_ready_go() == true {
                                        if ControlModule::get_stick_x(module_accessor) <= 1.0 {
                                            if KICKSTART_ANIM_BEGIN == false {
                                                StatusModule::change_status_request_from_script(boss_boma, *ITEM_DRACULA_STATUS_KIND_WAIT, true);
                                                KICKSTART_ANIM_BEGIN = true;
                                            }
                                            if StatusModule::status_kind(boss_boma) != *ITEM_DRACULA_STATUS_KIND_WAIT {
                                                StatusModule::change_status_request_from_script(boss_boma, *ITEM_DRACULA_STATUS_KIND_WAIT, true);
                                            }
                                        }
                                        if ControlModule::get_stick_x(module_accessor) >= 1.0 {
                                            if KICKSTART_ANIM_BEGIN == false {
                                                StatusModule::change_status_request_from_script(boss_boma, *ITEM_DRACULA_STATUS_KIND_WAIT, true);
                                                KICKSTART_ANIM_BEGIN = true;
                                            }
                                            if StatusModule::status_kind(boss_boma) != *ITEM_DRACULA_STATUS_KIND_WAIT {
                                                StatusModule::change_status_request_from_script(boss_boma, *ITEM_DRACULA_STATUS_KIND_WAIT, true);
                                            }
                                        }
                                        //Boss Control Movement
                                        if ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
                                            STOP_CONTROL_LOOP = false;
                                            StatusModule::change_status_request_from_script(boss_boma, *ITEM_DRACULA_STATUS_KIND_ATTACK_3WAY_START, true);
                                        }
                                        if ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_GUARD) {
                                            STOP_CONTROL_LOOP = false;
                                            StatusModule::change_status_request_from_script(boss_boma, *ITEM_DRACULA_STATUS_KIND_TELEPORT_START, true);
                                        }
                                        if ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
                                            STOP_CONTROL_LOOP = false;
                                            StatusModule::change_status_request_from_script(boss_boma, *ITEM_DRACULA_STATUS_KIND_ATTACK_FILL_START, true);
                                        }
                                        if ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW != 0 {
                                            STOP_CONTROL_LOOP = false;
                                            StatusModule::change_status_request_from_script(boss_boma, *ITEM_DRACULA_STATUS_KIND_ATTACK_RUSH_START, true);
                                        }
                                        if ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_HI != 0 {
                                            STOP_CONTROL_LOOP = false;
                                            StatusModule::change_status_request_from_script(boss_boma, *ITEM_DRACULA_STATUS_KIND_ATTACK_PILLAR_START, true);
                                        }
                                        if ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_S != 0 {
                                            STOP_CONTROL_LOOP = false;
                                            StatusModule::change_status_request_from_script(boss_boma, *ITEM_DRACULA_STATUS_KIND_ATTACK_STRAIGHT_START, true);
                                        }
                                        if ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW3 != 0 {
                                            STOP_CONTROL_LOOP = false;
                                            StatusModule::change_status_request_from_script(boss_boma, *ITEM_DRACULA_STATUS_KIND_ATTACK_TURN_3WAY_START, true);
                                        }
                                        if ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI3 != 0 {
                                            STOP_CONTROL_LOOP = false;
                                            StatusModule::change_status_request_from_script(boss_boma, *ITEM_DRACULA_STATUS_KIND_ATTACK_TURN_3WAY_START, true);
                                        }
                                        if ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S3 != 0 {
                                            STOP_CONTROL_LOOP = false;
                                            StatusModule::change_status_request_from_script(boss_boma, *ITEM_DRACULA_STATUS_KIND_ATTACK_TURN_3WAY_START, true);
                                        }
                                    }
                                }
                            }
                        }
                        if STOP_CONTROL_LOOP == true {
                            if TRANSFORMED_MODE == true {
                                if HAVE_ITEM  == true {
                                    if sv_information::is_ready_go() == true {
                                        //Boss Control Movement
                                        if ControlModule::get_stick_x(module_accessor) <= 0.001 {
                                            let pos = Vector3f{x: ControlModule::get_stick_x(module_accessor) * - 2.0 * ControlModule::get_stick_x(module_accessor), y: 0.0, z: 0.0};
                                            PostureModule::add_pos(boss_boma, &pos);
                                        }
                                    
                                        if ControlModule::get_stick_x(module_accessor) >= -0.001 {
                                            let pos = Vector3f{x: ControlModule::get_stick_x(module_accessor) * 2.0 * ControlModule::get_stick_x(module_accessor), y: 0.0, z: 0.0};
                                            PostureModule::add_pos(boss_boma, &pos);
                                        }
                                        if ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_JUMP) {
                                            STOP_CONTROL_LOOP = false;
                                            StatusModule::change_status_request_from_script(boss_boma, *ITEM_DRACULA2_STATUS_KIND_TURN, true);
                                        }
                                        if ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
                                            STOP_CONTROL_LOOP = false;
                                            StatusModule::change_status_request_from_script(boss_boma, *ITEM_DRACULA2_STATUS_KIND_STRIKE, true);
                                        }
                                        if ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_GUARD) {
                                            STOP_CONTROL_LOOP = false;
                                            StatusModule::change_status_request_from_script(boss_boma, *ITEM_DRACULA_STATUS_KIND_TELEPORT_START, true);
                                        }
                                        if ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
                                            STOP_CONTROL_LOOP = false;
                                            StatusModule::change_status_request_from_script(boss_boma, *ITEM_DRACULA2_STATUS_KIND_SLASH, true);
                                        }
                                        if ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW != 0 {
                                            STOP_CONTROL_LOOP = false;
                                            StatusModule::change_status_request_from_script(boss_boma, *ITEM_DRACULA2_STATUS_KIND_STEP_SLASH, true);
                                        }
                                        if ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_HI != 0 {
                                            STOP_CONTROL_LOOP = false;
                                            StatusModule::change_status_request_from_script(boss_boma, *ITEM_DRACULA2_STATUS_KIND_HOMING_SHOT_START, true);
                                        }
                                        if ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_S != 0 {
                                            STOP_CONTROL_LOOP = false;
                                            StatusModule::change_status_request_from_script(boss_boma, *ITEM_DRACULA2_STATUS_KIND_FRONT_JUMP, true);
                                        }
                                        if ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW3 != 0 {
                                            STOP_CONTROL_LOOP = false;
                                            StatusModule::change_status_request_from_script(boss_boma, *ITEM_DRACULA2_STATUS_KIND_FIRE_SHOT_START, true);
                                        }
                                        if ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI3 != 0 {
                                            STOP_CONTROL_LOOP = false;
                                            StatusModule::change_status_request_from_script(boss_boma, *ITEM_DRACULA2_STATUS_KIND_SQUASH_START, true);
                                        }
                                        if ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S3 != 0 {
                                            STOP_CONTROL_LOOP = false;
                                            StatusModule::change_status_request_from_script(boss_boma, *ITEM_DRACULA2_STATUS_KIND_BACK_JUMP, true);
                                        }
                                        if ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_APPEAL_HI) {
                                            STOP_CONTROL_LOOP = false;
                                            StatusModule::change_status_request_from_script(boss_boma, *ITEM_DRACULA2_STATUS_KIND_SHOCK_WAVE, true);
                                        }
                                        if ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_R) {
                                            STOP_CONTROL_LOOP = false;
                                            StatusModule::change_status_request_from_script(boss_boma, *ITEM_DRACULA2_STATUS_KIND_SLASH_THREE, true);
                                        }
                                        if ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_APPEAL_LW) {
                                            STOP_CONTROL_LOOP = false;
                                            StatusModule::change_status_request_from_script(boss_boma, *ITEM_DRACULA2_STATUS_KIND_TURN_SLASH, true);
                                        }
                                    }
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