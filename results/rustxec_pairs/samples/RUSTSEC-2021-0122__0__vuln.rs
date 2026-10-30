fn create_serialized_example_with_generated_code(builder: &mut flatbuffers::FlatBufferBuilder) {
    let mon = {
        let _ = builder.create_vector_of_strings(&[
            "these",
            "unused",
            "strings",
            "check",
            "the",
            "create_vector_of_strings",
            "function",
        ]);

        let s0 = builder.create_string("test1");
        let s1 = builder.create_string("test2");
        let fred_name = builder.create_string("Fred");

        // can't inline creation of this Vec3 because we refer to it by reference, so it must live
        // long enough to be used by MonsterArgs.
        let pos = my_game::example::Vec3::new(
            1.0,
            2.0,
            3.0,
            3.0,
            my_game::example::Color::Green,
            &my_game::example::Test::new(5i16, 6i8),
        );

        let args = my_game::example::MonsterArgs {
            hp: 80,
            mana: 150,
            name: Some(builder.create_string("MyMonster")),
            pos: Some(&pos),
            test_type: my_game::example::Any::Monster,
            test: Some(
                my_game::example::Monster::create(
                    builder,
                    &my_game::example::MonsterArgs {
                        name: Some(fred_name),
                        ..Default::default()
                    },
                )
                .as_union_value(),
            ),
            inventory: Some(builder.create_vector_direct(&[0u8, 1, 2, 3, 4][..])),
            test4: Some(builder.create_vector_direct(&[
                my_game::example::Test::new(10, 20),
                my_game::example::Test::new(30, 40),
            ])),
            testarrayofstring: Some(builder.create_vector(&[s0, s1])),
            ..Default::default()
        };
        my_game::example::Monster::create(builder, &args)
    };
    my_game::example::finish_monster_buffer(builder, mon);
}