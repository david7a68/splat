---
splat_schema: 1
splat_id: e6cc51d9-7cce-5e2c-a239-042f26305acb
splat_kind: rule_group
splat_key: dnd2024:species/tiefling
splat_name: Tiefling
splat_origin:
  key: dnd2024:reference/pages/086
splat_sources:
- title: SRD 5.2.1
  locator: pp. 86
  url: https://www.dndbeyond.com/srd
splat_rule_group:
  category: species
splat_rule:
  mechanic: dnd2024:species/tiefling
  implementation: dnd2024:species_tiefling
  parameters:
    creature_type: humanoid
    size_choice:
      count: 1
      options:
      - size: medium
        height_feet_approximate:
          minimum: 4
          maximum: 7
      - size: small
        height_feet_approximate:
          minimum: 3
          maximum: 4
    speed_feet: 30
    darkvision_feet: 60
    fiendish_legacy:
      choice:
        count: 1
        options:
        - id: abyssal
          level_1:
            damage_resistance: poison
            cantrip:
              key: dnd2024:spells/poison_spray
          level_3_spell:
            key: dnd2024:spells/ray_of_sickness
          level_5_spell:
            key: dnd2024:spells/hold_person
        - id: chthonic
          level_1:
            damage_resistance: necrotic
            cantrip:
              key: dnd2024:spells/chill_touch
          level_3_spell:
            key: dnd2024:spells/false_life
          level_5_spell:
            key: dnd2024:spells/ray_of_enfeeblement
        - id: infernal
          level_1:
            damage_resistance: fire
            cantrip:
              key: dnd2024:spells/fire_bolt
          level_3_spell:
            key: dnd2024:spells/hellish_rebuke
          level_5_spell:
            key: dnd2024:spells/darkness
      level_basis: character_level
      spellcasting_ability_choice:
        count: 1
        options:
        - intelligence
        - wisdom
        - charisma
      leveled_spell_grants:
        always_prepared: true
        each_spell_free_cast:
          spell_slot_cost: 0
          uses:
            maximum: 1
            recover:
              event: rest.long_rest_completed
              amount: all
        can_use_appropriate_spell_slots: true
    otherworldly_presence:
      cantrip:
        key: dnd2024:spells/thaumaturgy
      ability: chosen_fiendish_legacy_spellcasting_ability
splat_contracts:
- id: dnd2024:species_tiefling
  inputs:
    actor:
      type: participant
      required: true
    selections:
      required: false
      type: choice
  outputs:
  - feature_state
  - spell_grants
  events:
  - character.species_selected
  - character.level_gained
  - rest.long_rest_completed
  capabilities:
  - character_creation
  - spellcasting
  - senses
  - resistances
splat_conversion:
  status: specified
  category: species
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 86
  engine_gaps: &id001
  - dnd2024:species_tiefling
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_1.md
  specification_sha256: f695f2af4c6bcfecc4b42ebc6a0cd88c71b9fbcb11fb4fe03a8d8fb77ee58413
  line_span:
    first: 7420
    count: 30
splat_clauses:
- key: source_rules
  text: '# Tiefling Creature Type: Humanoid Size: Medium (about 4–7 feet tall) or Small (about 3–4 feet
    tall), chosen when you select this species Speed: 30 feet As a Tiefling, you have the following special
    traits. Darkvision. You have Darkvision with a range of 60 feet. Fiendish Legacy. You are the recipient
    of a legacy that grants you supernatural abilities. Choose a legacy from the Fiendish Legacies table.
    You gain the level 1 benefit of the chosen legacy. When you reach character levels 3 and 5, you learn
    a higher-level spell, as shown on the table. You always have that spell prepared. You can cast it
    once without a spell slot, and you regain the ability to cast it in that way when you finish a Long
    Rest. You can also cast the spell using any spell slots you have of the appropriate level. Intelligence,
    Wisdom, or Charisma is your spellcasting ability for the spells you cast with this trait (choose the
    ability when you select the legacy). Otherworldly Presence. You know the Thaumaturgy cantrip. When
    you cast it with this trait, the spell uses the same spellcasting ability you use for your Fiendish
    Legacy trait. Level 3 Level 5 Ray of Sickness Hold Person False Life Ray of Enfeeblement Hellish Rebuke
    Darkness'
  sha256: 82773c46be307aa61253c34b7cbd1430d4ae62731dfc44ae5e37e69b86f04ceb
  status: contract_specified
splat_requires_mechanics: *id001
---

# Tiefling

<!-- SRD page 86 -->

Creature Type: Humanoid\
Size: Medium (about 4–7 feet tall) or Small (about 3–4\
feet tall), chosen when you select this species\
Speed: 30 feet\
As a Tiefling, you have the following special traits.\
Darkvision. You have Darkvision with a range of\
60 feet.\
Fiendish Legacy. You are the recipient of a legacy\
that grants you supernatural abilities. Choose a leg-\
acy from the Fiendish Legacies table. You gain the\
level 1 benefit of the chosen legacy.\
When you reach character levels 3 and 5, you\
learn a higher-level spell, as shown on the table.\
You always have that spell prepared. You can cast it\
once without a spell slot, and you regain the ability\
to cast it in that way when you finish a Long Rest.\
You can also cast the spell using any spell slots you\
have of the appropriate level.\
Intelligence, Wisdom, or Charisma is your spell-\
casting ability for the spells you cast with this trait\
(choose the ability when you select the legacy).\
Otherworldly Presence. You know the Thauma-\
turgy cantrip. When you cast it with this trait, the\
spell uses the same spellcasting ability you use for\
your Fiendish Legacy trait.\
Level 3 Level 5\
Ray of Sickness Hold Person\
False Life Ray of Enfeeblement\
Hellish Rebuke Darkness\
