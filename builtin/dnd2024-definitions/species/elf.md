---
splat_schema: 1
splat_id: ce805c53-e8a6-5937-9b5f-57da4eee5039
splat_kind: rule_group
splat_key: dnd2024:species/elf
splat_name: Elf
splat_origin:
  key: dnd2024:reference/pages/084
splat_sources:
- title: SRD 5.2.1
  locator: pp. 84, 85
  url: https://www.dndbeyond.com/srd
splat_rule_group:
  category: species
splat_rule:
  mechanic: dnd2024:species/elf
  implementation: dnd2024:species_elf
  parameters:
    creature_type: humanoid
    size: medium
    height_feet_approximate:
      minimum: 5
      maximum: 6
    speed_feet: 30
    darkvision_feet: 60
    elven_lineage:
      choice:
        count: 1
        options:
        - id: drow
          level_1:
            darkvision_feet: 120
            cantrip:
              key: dnd2024:spells/dancing_lights
          level_3_spell:
            key: dnd2024:spells/faerie_fire
          level_5_spell:
            key: dnd2024:spells/darkness
        - id: high_elf
          level_1:
            cantrip:
              key: dnd2024:spells/prestidigitation
            replace_cantrip:
              event: rest.long_rest_completed
              list: wizard
              maximum: 1
          level_3_spell:
            key: dnd2024:spells/detect_magic
          level_5_spell:
            key: dnd2024:spells/misty_step
        - id: wood_elf
          level_1:
            speed_feet: 35
            cantrip:
              key: dnd2024:spells/druidcraft
          level_3_spell:
            key: dnd2024:spells/longstrider
          level_5_spell:
            key: dnd2024:spells/pass_without_trace
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
    fey_ancestry:
      saving_throw_advantage:
        avoid_or_end_condition: charmed
    keen_senses:
      skill_proficiency_choice:
        count: 1
        options:
        - insight
        - perception
        - survival
    trance:
      sleep_required: false
      magical_sleep_immune: true
      long_rest_hours: 4
      required_activity: trancelike_meditation
      conscious_during_rest: true
splat_contracts:
- id: dnd2024:species_elf
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
  - proficiencies
  events:
  - character.species_selected
  - character.level_gained
  - rest.long_rest_completed
  - saving_throw.pending
  capabilities:
  - character_creation
  - spellcasting
  - senses
  - proficiencies
  - rest
splat_conversion:
  status: specified
  category: species
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 84
  - 85
  engine_gaps: &id001
  - dnd2024:species_elf
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_1.md
  specification_sha256: f7a9e928e62ce3d8d3728bd3ad2e2e080eea2bc502f525a58da9d82e73d59a2e
  line_span:
    first: 7237
    count: 39
splat_clauses:
- key: source_rules
  text: '# Elf Creature Type: Humanoid Size: Medium (about 5–6 feet tall) Speed: 30 feet As an Elf, you
    have these special traits. Darkvision. You have Darkvision with a range of 60 feet. Elven Lineage.
    You are part of a lineage that grants you supernatural abilities. Choose a lineage from the Elven
    Lineages table. You gain the level 1 benefit of that lineage. When you reach character levels 3 and
    5, you learn a higher-level spell, as shown on the table. You always have that spell prepared. You
    can cast it once without a spell slot, and you regain the ability to cast it in that way when you
    finish a Long Rest. You can also cast the spell using any spell slots you have of the appropriate
    level. Elven Lineages Lineage Level 1 Drow The range of your Darkvision increases to 120 feet. You
    also know the Dancing Lights cantrip. High Elf You know the Prestidigitation cantrip. Whenever you
    finish a Long Rest, you can replace that cantrip with a different cantrip from the Wizard spell list.
    Wood Elf Your Speed increases to 35 feet. You also know the Druidcraft cantrip. Intelligence, Wisdom,
    or Charisma is your spellcasting ability for the spells you cast with this trait (choose the ability
    when you select the lineage). Fey Ancestry. You have Advantage on saving throws you make to avoid
    or end the Charmed condition. Keen Senses. You have proficiency in the Insight, Perception, or Survival
    skill. Trance. You don’t need to sleep, and magic can’t put you to sleep. You can finish a Long Rest
    in 4 hours if you spend those hours in a trancelike meditation, during which you retain consciousness.'
  sha256: d36702972c8c812a8afef74b2a1d9ef343ece1f9223b65c9c42b70df03f92b4e
  status: contract_specified
splat_requires_mechanics: *id001
---

# Elf

<!-- SRD page 84 -->

Creature Type: Humanoid\
Size: Medium (about 5–6 feet tall)\
Speed: 30 feet\
As an Elf, you have these special traits.\
Darkvision. You have Darkvision with a range of\
60 feet.\
Elven Lineage. You are part of a lineage that\
grants you supernatural abilities. Choose a lineage\
from the Elven Lineages table. You gain the level 1\
benefit of that lineage.\
When you reach character levels 3 and 5, you\
learn a higher-level spell, as shown on the table.\
You always have that spell prepared. You can cast it\
once without a spell slot, and you regain the ability\
to cast it in that way when you finish a Long Rest.\
You can also cast the spell using any spell slots you\
have of the appropriate level.\

<!-- SRD page 85 -->

Elven Lineages\
Lineage Level 1\
Drow The range of your Darkvision increases to 120 feet.\
You also know the Dancing Lights cantrip.\
High Elf You know the Prestidigitation cantrip. Whenever you finish\
a Long Rest, you can replace that cantrip with a different\
cantrip from the Wizard spell list.\
Wood Elf Your Speed increases to 35 feet. You also know the\
Druidcraft cantrip.\
Intelligence, Wisdom, or Charisma is your spell-\
casting ability for the spells you cast with this trait\
(choose the ability when you select the lineage).\
Fey Ancestry. You have Advantage on saving\
throws you make to avoid or end the Charmed\
condition.\
Keen Senses. You have proficiency in the Insight,\
Perception, or Survival skill.\
Trance. You don’t need to sleep, and magic can’t\
put you to sleep. You can finish a Long Rest in 4\
hours if you spend those hours in a trancelike medi-\
tation, during which you retain consciousness.\
