fn main() {
    cc::Build::new()
        .file("doom/am_map.c")
        .file("doom/d_items.c")
        .file("doom/d_main.c")
        .file("doom/d_net.c")
        .file("doom/doomdef.c")
        .file("doom/doomstat.c")
        .file("doom/dstrings.c")
        .file("doom/f_finale.c")
        .file("doom/f_wipe.c")
        .file("doom/g_game.c")
        .file("doom/hu_lib.c")
        .file("doom/hu_stuff.c")
        .file("doom/i_system.c")
        .file("doom/info.c")
        .file("doom/m_argv.c")
        .file("doom/m_bbox.c")
        .file("doom/m_cheat.c")
        .file("doom/m_fixed.c")
        .file("doom/m_menu.c")
        .file("doom/m_misc.c")
        .file("doom/m_random.c")
        .file("doom/m_swap.c")
        .file("doom/p_ceilng.c")
        .file("doom/p_doors.c")
        .file("doom/p_enemy.c")
        .file("doom/p_floor.c")
        .file("doom/p_inter.c")
        .file("doom/p_lights.c")
        .file("doom/p_map.c")
        .file("doom/p_maputl.c")
        .file("doom/p_mobj.c")
        .file("doom/p_plats.c")
        .file("doom/p_pspr.c")
        .file("doom/p_saveg.c")
        .file("doom/p_setup.c")
        .file("doom/p_sight.c")
        .file("doom/p_spec.c")
        .file("doom/p_switch.c")
        .file("doom/p_telept.c")
        .file("doom/p_tick.c")
        .file("doom/p_user.c")
        .file("doom/r_bsp.c")
        .file("doom/r_data.c")
        .file("doom/r_draw.c")
        .file("doom/r_main.c")
        .file("doom/r_plane.c")
        .file("doom/r_segs.c")
        .file("doom/r_sky.c")
        .file("doom/r_things.c")
        .file("doom/sounds.c")
        .file("doom/s_sound.c")
        .file("doom/st_lib.c")
        .file("doom/st_stuff.c")
        .file("doom/tables.c")
        .file("doom/v_video.c")
        .file("doom/wi_stuff.c")
        .file("doom/w_wad.c")
        .file("doom/z_zone.c")
        .define("NORMALUNIX", "")
        .define("LINUX", "")
        .warnings(false)
        .compile("doom")
}