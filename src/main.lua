function setup()
    print("hello from lua!")
    return {}
end

function update(game, dt)
    mq_clear_background({0.8, 0.6, 0.0, 1.0});

    mq_draw_line({40.0, 40.0}, {100.0, 200.0}, 15.0, {0.0, 0.0, 1.0, 1.0});
    mq_draw_rect_wh({mq_screen_width() / 2.0 - 60.0, 100.0}, {120.0, 60.0}, {0.0, 1.0, 0.0, 1.0});

    return true
end


return { setup=setup, update=update }
