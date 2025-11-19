use std::{
    fs::{self, File},
    io::BufReader,
    path::Path,
};

use rodio::{OutputStream, Sink};
use serde::Deserialize;
use winput::{Action, message_loop};

#[derive(Deserialize)]
struct KeyboardConfig {
    a: Box<Path>,
    a_up: Box<Path>,
    b: Box<Path>,
    b_up: Box<Path>,
    c: Box<Path>,
    c_up: Box<Path>,
    d: Box<Path>,
    d_up: Box<Path>,
    e: Box<Path>,
    e_up: Box<Path>,
    f: Box<Path>,
    f_up: Box<Path>,
    g: Box<Path>,
    g_up: Box<Path>,
    h: Box<Path>,
    h_up: Box<Path>,
    i: Box<Path>,
    i_up: Box<Path>,
    j: Box<Path>,
    j_up: Box<Path>,
    k: Box<Path>,
    k_up: Box<Path>,
    l: Box<Path>,
    l_up: Box<Path>,
    m: Box<Path>,
    m_up: Box<Path>,
    n: Box<Path>,
    n_up: Box<Path>,
    o: Box<Path>,
    o_up: Box<Path>,
    p: Box<Path>,
    p_up: Box<Path>,
    q: Box<Path>,
    q_up: Box<Path>,
    r: Box<Path>,
    r_up: Box<Path>,
    s: Box<Path>,
    s_up: Box<Path>,
    t: Box<Path>,
    t_up: Box<Path>,
    u: Box<Path>,
    u_up: Box<Path>,
    v: Box<Path>,
    v_up: Box<Path>,
    w: Box<Path>,
    w_up: Box<Path>,
    x: Box<Path>,
    x_up: Box<Path>,
    y: Box<Path>,
    y_up: Box<Path>,
    z: Box<Path>,
    z_up: Box<Path>,
    enter: Box<Path>,
    enter_up: Box<Path>,
    shift: Box<Path>,
    shift_up: Box<Path>,
    lshift: Box<Path>,
    lshift_up: Box<Path>,
    rshift: Box<Path>,
    rshift_up: Box<Path>,
    esc: Box<Path>,
    esc_up: Box<Path>,
    space: Box<Path>,
    space_up: Box<Path>,
    delete: Box<Path>,
    delete_up: Box<Path>,
    backspace: Box<Path>,
    backspace_up: Box<Path>,
    leftarrow: Box<Path>,
    leftarrow_up: Box<Path>,
    rightarrow: Box<Path>,
    rightarrow_up: Box<Path>,
    uparrow: Box<Path>,
    uparrow_up: Box<Path>,
    downarrow: Box<Path>,
    downarrow_up: Box<Path>,
    f1: Box<Path>,
    f1_up: Box<Path>,
    f2: Box<Path>,
    f2_up: Box<Path>,
    f3: Box<Path>,
    f3_up: Box<Path>,
    f4: Box<Path>,
    f4_up: Box<Path>,
    f5: Box<Path>,
    f5_up: Box<Path>,
    f6: Box<Path>,
    f6_up: Box<Path>,
    f7: Box<Path>,
    f7_up: Box<Path>,
    f8: Box<Path>,
    f8_up: Box<Path>,
    f9: Box<Path>,
    f9_up: Box<Path>,
    f10: Box<Path>,
    f10_up: Box<Path>,
    f11: Box<Path>,
    f11_up: Box<Path>,
    f12: Box<Path>,
    f12_up: Box<Path>,
    f13: Box<Path>,
    f13_up: Box<Path>,
    f14: Box<Path>,
    f14_up: Box<Path>,
    f15: Box<Path>,
    f15_up: Box<Path>,
    f16: Box<Path>,
    f16_up: Box<Path>,
    f17: Box<Path>,
    f17_up: Box<Path>,
    f18: Box<Path>,
    f18_up: Box<Path>,
    f19: Box<Path>,
    f19_up: Box<Path>,
    f20: Box<Path>,
    f20_up: Box<Path>,
    f21: Box<Path>,
    f21_up: Box<Path>,
    f22: Box<Path>,
    f22_up: Box<Path>,
    f23: Box<Path>,
    f23_up: Box<Path>,
    f24: Box<Path>,
    f24_up: Box<Path>,
    comma: Box<Path>,
    comma_up: Box<Path>,
    tab: Box<Path>,
    tab_up: Box<Path>,
    alt: Box<Path>,
    alt_up: Box<Path>,
    ctrl: Box<Path>,
    ctrl_up: Box<Path>,
    lctrl: Box<Path>,
    lctrl_up: Box<Path>,
    rctrl: Box<Path>,
    rctrl_up: Box<Path>,
    home: Box<Path>,
    home_up: Box<Path>,
    end: Box<Path>,
    end_up: Box<Path>,
    key_0: Box<Path>,
    key_0_up: Box<Path>,
    key_1: Box<Path>,
    key_1_up: Box<Path>,
    key_2: Box<Path>,
    key_2_up: Box<Path>,
    key_3: Box<Path>,
    key_3_up: Box<Path>,
    key_4: Box<Path>,
    key_4_up: Box<Path>,
    key_5: Box<Path>,
    key_5_up: Box<Path>,
    key_6: Box<Path>,
    key_6_up: Box<Path>,
    key_7: Box<Path>,
    key_7_up: Box<Path>,
    key_8: Box<Path>,
    key_8_up: Box<Path>,
    key_9: Box<Path>,
    key_9_up: Box<Path>,
    numpad_0: Box<Path>,
    numpad_0_up: Box<Path>,
    numpad_1: Box<Path>,
    numpad_1_up: Box<Path>,
    numpad_2: Box<Path>,
    numpad_2_up: Box<Path>,
    numpad_3: Box<Path>,
    numpad_3_up: Box<Path>,
    numpad_4: Box<Path>,
    numpad_4_up: Box<Path>,
    numpad_5: Box<Path>,
    numpad_5_up: Box<Path>,
    numpad_6: Box<Path>,
    numpad_6_up: Box<Path>,
    numpad_7: Box<Path>,
    numpad_7_up: Box<Path>,
    numpad_8: Box<Path>,
    numpad_8_up: Box<Path>,
    numpad_9: Box<Path>,
    numpad_9_up: Box<Path>,
    right_win: Box<Path>,
    right_win_up: Box<Path>,
    left_win: Box<Path>,
    left_win_up: Box<Path>,
    period: Box<Path>,
    period_up: Box<Path>,
    oem_1: Box<Path>,
    oem_1_up: Box<Path>,
    oem_2: Box<Path>,
    oem_2_up: Box<Path>,
    oem_3: Box<Path>,
    oem_3_up: Box<Path>,
    oem_4: Box<Path>,
    oem_4_up: Box<Path>,
    oem_5: Box<Path>,
    oem_5_up: Box<Path>,
    oem_6: Box<Path>,
    oem_6_up: Box<Path>,
    oem_7: Box<Path>,
    oem_7_up: Box<Path>,
    oem_8: Box<Path>,
    oem_8_up: Box<Path>,
    oem_102: Box<Path>,
    oem_102_up: Box<Path>,
    plus: Box<Path>,
    plus_up: Box<Path>,
    minus: Box<Path>,
    minus_up: Box<Path>,
    multiply: Box<Path>,
    multiply_up: Box<Path>,
    subtract: Box<Path>,
    subtract_up: Box<Path>,
    capslock: Box<Path>,
    capslock_up: Box<Path>,
    numlock: Box<Path>,
    numlock_up: Box<Path>,
}

fn keyboard_bindings(
    keyboard_config: &KeyboardConfig,
    vk: winput::Vk,
    action: Action,
) -> Option<(OutputStream, Sink)> {
    let mut opt_handle = None;
    println!("vk: {vk:?}");
    (winput::Vk::A == vk && Action::Press == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.a));
    });
    (winput::Vk::A == vk && Action::Release == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.a_up));
    });
    (winput::Vk::B == vk && Action::Press == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.b));
    });
    (winput::Vk::B == vk && Action::Release == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.b_up));
    });
    (winput::Vk::C == vk && Action::Press == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.c));
    });
    (winput::Vk::C == vk && Action::Release == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.c_up));
    });
    (winput::Vk::D == vk && Action::Press == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.d));
    });
    (winput::Vk::D == vk && Action::Release == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.d_up));
    });
    (winput::Vk::E == vk && Action::Press == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.e));
    });
    (winput::Vk::E == vk && Action::Release == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.e_up));
    });
    (winput::Vk::F == vk && Action::Press == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.f));
    });
    (winput::Vk::F == vk && Action::Release == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.f_up));
    });
    (winput::Vk::G == vk && Action::Press == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.g));
    });
    (winput::Vk::G == vk && Action::Release == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.g_up));
    });
    (winput::Vk::H == vk && Action::Press == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.h));
    });
    (winput::Vk::H == vk && Action::Release == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.h_up));
    });
    (winput::Vk::I == vk && Action::Press == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.i));
    });
    (winput::Vk::I == vk && Action::Release == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.i_up));
    });
    (winput::Vk::J == vk && Action::Press == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.j));
    });
    (winput::Vk::J == vk && Action::Release == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.j_up));
    });
    (winput::Vk::K == vk && Action::Press == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.k));
    });
    (winput::Vk::K == vk && Action::Release == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.k_up));
    });
    (winput::Vk::L == vk && Action::Press == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.l));
    });
    (winput::Vk::L == vk && Action::Release == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.l_up));
    });
    (winput::Vk::M == vk && Action::Press == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.m));
    });
    (winput::Vk::M == vk && Action::Release == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.m_up));
    });
    (winput::Vk::N == vk && Action::Press == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.n));
    });
    (winput::Vk::N == vk && Action::Release == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.n_up));
    });
    (winput::Vk::O == vk && Action::Press == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.o));
    });
    (winput::Vk::O == vk && Action::Release == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.o_up));
    });
    (winput::Vk::P == vk && Action::Press == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.p));
    });
    (winput::Vk::P == vk && Action::Release == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.p_up));
    });
    (winput::Vk::Q == vk && Action::Press == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.q));
    });
    (winput::Vk::Q == vk && Action::Release == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.q_up));
    });
    (winput::Vk::R == vk && Action::Press == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.r));
    });
    (winput::Vk::R == vk && Action::Release == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.r_up));
    });
    (winput::Vk::S == vk && Action::Press == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.s));
    });
    (winput::Vk::S == vk && Action::Release == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.s_up));
    });
    (winput::Vk::T == vk && Action::Press == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.t));
    });
    (winput::Vk::T == vk && Action::Release == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.t_up));
    });
    (winput::Vk::U == vk && Action::Press == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.u));
    });
    (winput::Vk::U == vk && Action::Release == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.u_up));
    });
    (winput::Vk::V == vk && Action::Press == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.v));
    });
    (winput::Vk::V == vk && Action::Release == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.v_up));
    });
    (winput::Vk::W == vk && Action::Press == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.w));
    });
    (winput::Vk::W == vk && Action::Release == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.w_up));
    });
    (winput::Vk::X == vk && Action::Press == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.x));
    });
    (winput::Vk::X == vk && Action::Release == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.x_up));
    });
    (winput::Vk::Y == vk && Action::Press == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.y));
    });
    (winput::Vk::Y == vk && Action::Release == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.y_up));
    });
    (winput::Vk::Z == vk && Action::Press == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.z));
    });
    (winput::Vk::Z == vk && Action::Release == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.z_up));
    });
    (winput::Vk::Enter == vk && Action::Press == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.enter));
    });
    (winput::Vk::Enter == vk && Action::Release == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.enter_up));
    });
    (winput::Vk::Escape == vk && Action::Press == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.esc));
    });
    (winput::Vk::Escape == vk && Action::Release == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.esc_up));
    });
    (winput::Vk::Shift == vk && Action::Press == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.shift));
    });
    (winput::Vk::Shift == vk && Action::Release == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.shift_up));
    });
    (winput::Vk::LeftShift == vk && Action::Press == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.lshift));
    });
    (winput::Vk::LeftShift == vk && Action::Release == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.lshift_up));
    });
    (winput::Vk::RightShift == vk && Action::Press == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.rshift));
    });
    (winput::Vk::RightShift == vk && Action::Release == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.rshift_up));
    });
    (winput::Vk::Space == vk && Action::Press == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.space));
    });
    (winput::Vk::Space == vk && Action::Release == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.space_up));
    });
    (winput::Vk::Delete == vk && Action::Press == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.delete));
    });
    (winput::Vk::Delete == vk && Action::Release == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.delete_up));
    });
    (winput::Vk::Backspace == vk && Action::Press == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.backspace));
    });
    (winput::Vk::Backspace == vk && Action::Release == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.backspace_up));
    });
    (winput::Vk::LeftArrow == vk && Action::Press == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.leftarrow));
    });
    (winput::Vk::LeftArrow == vk && Action::Release == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.leftarrow_up));
    });
    (winput::Vk::RightArrow == vk && Action::Press == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.rightarrow));
    });
    (winput::Vk::RightArrow == vk && Action::Release == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.rightarrow_up));
    });
    (winput::Vk::UpArrow == vk && Action::Press == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.uparrow));
    });
    (winput::Vk::UpArrow == vk && Action::Release == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.uparrow_up));
    });
    (winput::Vk::DownArrow == vk && Action::Press == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.downarrow));
    });
    (winput::Vk::DownArrow == vk && Action::Release == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.downarrow_up));
    });
    (winput::Vk::F1 == vk && Action::Press == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.f1));
    });
    (winput::Vk::F1 == vk && Action::Release == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.f1_up));
    });
    (winput::Vk::F2 == vk && Action::Press == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.f2));
    });
    (winput::Vk::F2 == vk && Action::Release == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.f2_up));
    });
    (winput::Vk::F3 == vk && Action::Press == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.f3));
    });
    (winput::Vk::F3 == vk && Action::Release == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.f3_up));
    });
    (winput::Vk::F4 == vk && Action::Press == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.f4));
    });
    (winput::Vk::F4 == vk && Action::Release == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.f4_up));
    });
    (winput::Vk::F5 == vk && Action::Press == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.f5));
    });
    (winput::Vk::F5 == vk && Action::Release == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.f5_up));
    });
    (winput::Vk::F6 == vk && Action::Press == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.f6));
    });
    (winput::Vk::F6 == vk && Action::Release == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.f6_up));
    });
    (winput::Vk::F7 == vk && Action::Press == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.f7));
    });
    (winput::Vk::F7 == vk && Action::Release == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.f7_up));
    });
    (winput::Vk::F8 == vk && Action::Press == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.f8));
    });
    (winput::Vk::F8 == vk && Action::Release == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.f8_up));
    });
    (winput::Vk::F9 == vk && Action::Press == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.f9));
    });
    (winput::Vk::F9 == vk && Action::Release == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.f9_up));
    });
    (winput::Vk::F10 == vk && Action::Press == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.f10));
    });
    (winput::Vk::F10 == vk && Action::Release == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.f10_up));
    });
    (winput::Vk::F11 == vk && Action::Press == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.f11));
    });
    (winput::Vk::F11 == vk && Action::Release == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.f11_up));
    });
    (winput::Vk::F12 == vk && Action::Press == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.f12));
    });
    (winput::Vk::F12 == vk && Action::Release == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.f12_up));
    });
    (winput::Vk::F13 == vk && Action::Press == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.f13));
    });
    (winput::Vk::F13 == vk && Action::Release == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.f13_up));
    });
    (winput::Vk::F14 == vk && Action::Press == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.f14));
    });
    (winput::Vk::F14 == vk && Action::Release == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.f14_up));
    });
    (winput::Vk::F15 == vk && Action::Press == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.f15));
    });
    (winput::Vk::F15 == vk && Action::Release == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.f15_up));
    });
    (winput::Vk::F16 == vk && Action::Press == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.f16));
    });
    (winput::Vk::F16 == vk && Action::Release == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.f16_up));
    });
    (winput::Vk::F17 == vk && Action::Press == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.f17));
    });
    (winput::Vk::F17 == vk && Action::Release == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.f17_up));
    });
    (winput::Vk::F18 == vk && Action::Press == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.f18));
    });
    (winput::Vk::F18 == vk && Action::Release == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.f18_up));
    });
    (winput::Vk::F19 == vk && Action::Press == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.f19));
    });
    (winput::Vk::F19 == vk && Action::Release == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.f19_up));
    });
    (winput::Vk::F20 == vk && Action::Press == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.f20));
    });
    (winput::Vk::F20 == vk && Action::Release == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.f20_up));
    });
    (winput::Vk::F21 == vk && Action::Press == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.f21));
    });
    (winput::Vk::F21 == vk && Action::Release == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.f21_up));
    });
    (winput::Vk::F22 == vk && Action::Press == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.f22));
    });
    (winput::Vk::F22 == vk && Action::Release == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.f22_up));
    });
    (winput::Vk::F23 == vk && Action::Press == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.f23));
    });
    (winput::Vk::F23 == vk && Action::Release == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.f23_up));
    });
    (winput::Vk::F24 == vk && Action::Press == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.f24));
    });
    (winput::Vk::F24 == vk && Action::Release == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.f24_up));
    });
    (winput::Vk::Comma == vk && Action::Press == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.comma));
    });
    (winput::Vk::Comma == vk && Action::Release == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.comma_up));
    });
    (winput::Vk::Tab == vk && Action::Press == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.tab));
    });
    (winput::Vk::Tab == vk && Action::Release == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.tab_up));
    });
    (winput::Vk::Alt == vk && Action::Press == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.alt));
    });
    (winput::Vk::Alt == vk && Action::Release == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.alt_up));
    });
    (winput::Vk::LeftControl == vk && Action::Press == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.lctrl));
    });
    (winput::Vk::LeftControl == vk && Action::Release == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.lctrl_up));
    });
    (winput::Vk::Control == vk && Action::Press == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.ctrl));
    });
    (winput::Vk::Control == vk && Action::Release == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.ctrl_up));
    });
    (winput::Vk::RightControl == vk && Action::Press == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.rctrl));
    });
    (winput::Vk::RightControl == vk && Action::Release == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.rctrl_up));
    });
    (winput::Vk::Home == vk && Action::Press == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.home));
    });
    (winput::Vk::Home == vk && Action::Release == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.home_up));
    });
    (winput::Vk::End == vk && Action::Press == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.end));
    });
    (winput::Vk::End == vk && Action::Release == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.end_up));
    });
    (winput::Vk::_0 == vk && Action::Press == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.key_0));
    });
    (winput::Vk::_0 == vk && Action::Release == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.key_0_up));
    });
    (winput::Vk::_1 == vk && Action::Press == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.key_1));
    });
    (winput::Vk::_1 == vk && Action::Release == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.key_1_up));
    });
    (winput::Vk::_2 == vk && Action::Press == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.key_2));
    });
    (winput::Vk::_2 == vk && Action::Release == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.key_2_up));
    });
    (winput::Vk::_3 == vk && Action::Press == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.key_3));
    });
    (winput::Vk::_3 == vk && Action::Release == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.key_3_up));
    });
    (winput::Vk::_4 == vk && Action::Press == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.key_4));
    });
    (winput::Vk::_4 == vk && Action::Release == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.key_4_up));
    });
    (winput::Vk::_5 == vk && Action::Press == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.key_5));
    });
    (winput::Vk::_5 == vk && Action::Release == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.key_5_up));
    });
    (winput::Vk::_6 == vk && Action::Press == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.key_6));
    });
    (winput::Vk::_6 == vk && Action::Release == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.key_6_up));
    });
    (winput::Vk::_7 == vk && Action::Press == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.key_7));
    });
    (winput::Vk::_7 == vk && Action::Release == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.key_7_up));
    });
    (winput::Vk::_8 == vk && Action::Press == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.key_8));
    });
    (winput::Vk::_8 == vk && Action::Release == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.key_8_up));
    });
    (winput::Vk::_9 == vk && Action::Press == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.key_9));
    });
    (winput::Vk::_9 == vk && Action::Release == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.key_9_up));
    });
    (winput::Vk::Numpad0 == vk && Action::Press == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.numpad_0));
    });
    (winput::Vk::Numpad0 == vk && Action::Release == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.numpad_0_up));
    });
    (winput::Vk::Numpad1 == vk && Action::Press == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.numpad_1));
    });
    (winput::Vk::Numpad1 == vk && Action::Release == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.numpad_1_up));
    });
    (winput::Vk::Numpad2 == vk && Action::Press == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.numpad_2));
    });
    (winput::Vk::Numpad2 == vk && Action::Release == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.numpad_2_up));
    });
    (winput::Vk::Numpad3 == vk && Action::Press == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.numpad_3));
    });
    (winput::Vk::Numpad3 == vk && Action::Release == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.numpad_3_up));
    });
    (winput::Vk::Numpad4 == vk && Action::Press == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.numpad_4));
    });
    (winput::Vk::Numpad4 == vk && Action::Release == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.numpad_4_up));
    });
    (winput::Vk::Numpad5 == vk && Action::Press == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.numpad_5));
    });
    (winput::Vk::Numpad5 == vk && Action::Release == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.numpad_5_up));
    });
    (winput::Vk::Numpad6 == vk && Action::Press == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.numpad_6));
    });
    (winput::Vk::Numpad6 == vk && Action::Release == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.numpad_6_up));
    });
    (winput::Vk::Numpad7 == vk && Action::Press == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.numpad_7));
    });
    (winput::Vk::Numpad7 == vk && Action::Release == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.numpad_7_up));
    });
    (winput::Vk::Numpad8 == vk && Action::Press == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.numpad_8));
    });
    (winput::Vk::Numpad8 == vk && Action::Release == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.numpad_8_up));
    });
    (winput::Vk::Numpad9 == vk && Action::Press == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.numpad_9));
    });
    (winput::Vk::Numpad9 == vk && Action::Release == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.numpad_9_up));
    });
    (winput::Vk::RightWin == vk && Action::Press == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.right_win));
    });
    (winput::Vk::RightWin == vk && Action::Release == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.right_win_up));
    });
    (winput::Vk::LeftWin == vk && Action::Press == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.left_win));
    });
    (winput::Vk::LeftWin == vk && Action::Release == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.left_win_up));
    });
    (winput::Vk::Oem102 == vk && Action::Press == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.oem_102));
    });
    (winput::Vk::Oem102 == vk && Action::Release == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.oem_102_up));
    });
    (winput::Vk::Oem2 == vk && Action::Press == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.oem_2));
    });
    (winput::Vk::Oem2 == vk && Action::Release == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.oem_2_up));
    });
    (winput::Vk::Period == vk && Action::Press == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.period));
    });
    (winput::Vk::Period == vk && Action::Release == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.period_up));
    });
    (winput::Vk::Oem1 == vk && Action::Press == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.oem_1));
    });
    (winput::Vk::Oem1 == vk && Action::Release == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.oem_1_up));
    });
    (winput::Vk::Oem2 == vk && Action::Press == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.oem_2));
    });
    (winput::Vk::Oem2 == vk && Action::Release == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.oem_2_up));
    });
    (winput::Vk::Oem3 == vk && Action::Press == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.oem_3));
    });
    (winput::Vk::Oem3 == vk && Action::Release == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.oem_3_up));
    });
    (winput::Vk::Oem4 == vk && Action::Press == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.oem_4));
    });
    (winput::Vk::Oem4 == vk && Action::Release == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.oem_4_up));
    });
    (winput::Vk::Oem5 == vk && Action::Press == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.oem_5));
    });
    (winput::Vk::Oem5 == vk && Action::Release == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.oem_5_up));
    });
    (winput::Vk::Oem6 == vk && Action::Press == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.oem_6));
    });
    (winput::Vk::Oem6 == vk && Action::Release == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.oem_6_up));
    });
    (winput::Vk::Oem7 == vk && Action::Press == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.oem_7));
    });
    (winput::Vk::Oem7 == vk && Action::Release == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.oem_7_up));
    });
    (winput::Vk::Oem8 == vk && Action::Press == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.oem_8));
    });
    (winput::Vk::Oem8 == vk && Action::Release == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.oem_8_up));
    });
    (winput::Vk::Plus == vk && Action::Press == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.plus));
    });
    (winput::Vk::Plus == vk && Action::Release == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.plus_up));
    });
    (winput::Vk::Minus == vk && Action::Press == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.minus));
    });
    (winput::Vk::Minus == vk && Action::Release == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.minus_up));
    });
    (winput::Vk::Multiply == vk && Action::Press == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.multiply));
    });
    (winput::Vk::Multiply == vk && Action::Release == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.multiply_up));
    });
    (winput::Vk::Subtract == vk && Action::Press == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.subtract));
    });
    (winput::Vk::Subtract == vk && Action::Release == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.subtract_up));
    });
    (winput::Vk::CapsLock == vk && Action::Press == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.capslock));
    });
    (winput::Vk::CapsLock == vk && Action::Release == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.capslock_up));
    });
    (winput::Vk::Numlock == vk && Action::Press == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.numlock));
    });
    (winput::Vk::Numlock == vk && Action::Release == action).then(|| {
        opt_handle = Some(play_audio(&keyboard_config.numlock_up));
    });
    opt_handle
}

fn play_audio(file: &Box<Path>) -> (OutputStream, Sink) {
    let file = std::env::home_dir()
        .expect("Could not get HomeDir")
        .join(".config/Soundput/")
        .join(file);
    let mut stream_handle =
        rodio::OutputStreamBuilder::open_default_stream().expect("Open default audio stream");
    let file = BufReader::new(
        File::open(&file).expect(&format!("Could not open file {}", file.to_string_lossy())),
    );
    stream_handle.log_on_drop(false);
    let sink = rodio::play(&stream_handle.mixer(), file).expect("Could not play audio");
    (stream_handle, sink)
}

fn get_default_config() -> KeyboardConfig {
    let mut toml_path = std::env::home_dir().expect("Could not get HomeDir");
    toml_path.push(".config/Soundput/config.toml");
    assert!(toml_path.exists());
    let file = fs::read(toml_path)
        .expect("Could not read config.toml")
        .iter()
        .map(|c| *c as char)
        .collect::<String>();
    let keyboard_config: KeyboardConfig =
        toml::from_str(&file).expect("Config.toml is not to a proper format!");
    keyboard_config
}

fn main() {
    let keyboard_config = get_default_config();
    let receiver = message_loop::start().expect("Could not start message loop");
    let mut opt_handle: Vec<Option<(OutputStream, Sink)>> = Vec::default();
    loop {
        match receiver.try_next_event() {
            Some(message_loop::Event::Keyboard {
                vk,
                scan_code: _,
                action,
            }) => {
                opt_handle.push(keyboard_bindings(&keyboard_config, vk, action));
                opt_handle.iter_mut().for_each(|opt_handle| {
                    if let Some((_, sink)) = opt_handle {
                        if sink.empty() {
                            *opt_handle = None;
                        }
                    }
                });
                opt_handle.pop_if(|v| v.is_none());
            }
            _ => {}
        }
    }
}
