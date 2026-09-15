---
splat_schema: 1
splat_id: 0a38a9c2-4dbf-5f8b-8ed9-1ea2346a6e28
splat_kind: item
splat_key: dnd2024:items/necklace_of_prayer_beads
splat_name: Necklace of Prayer Beads
splat_origin:
  key: dnd2024:reference/magic_item/necklace_of_prayer_beads
splat_sources:
- title: SRD 5.2.1
  locator: pp. 233
  url: https://www.dndbeyond.com/srd
splat_item:
  category: wondrous_item
  rarity: rare
  equipment:
    attunement: required
    attunement_requirement_text: a Cleric, Druid, or Paladin
splat_rule:
  mechanic: dnd2024:items/necklace_of_prayer_beads
  implementation: dnd2024:item_necklace_of_prayer_beads
  parameters:
    equipped: worn
    attunement_classes:
    - cleric
    - druid
    - paladin
    magic_bead_count:
      roll:
        count: 1
        sides: 4
        add: 2
    removed_magic_bead: nonmagical
    bead_type_selection: gm_choice_or_roll
    duplicate_types_allowed: true
    bead_die: 20
    beads:
    - roll:
      - 1
      - 6
      name: blessing
      cast:
        spell:
          key: dnd2024:spells/bless
        action: bonus_action
        save_dc: user.spell_save_dc
        level: 1
      uses:
        maximum: 1
        recovery_event: time.dawn
      usage_pool: per_bead
    - roll:
      - 7
      - 12
      name: curing
      cast:
        spell:
          key: dnd2024:spells/cure_wounds
        action: bonus_action
        save_dc: user.spell_save_dc
        level: 2
      uses:
        maximum: 1
        recovery_event: time.dawn
      usage_pool: per_bead
    - roll:
      - 13
      - 16
      name: favor
      cast:
        spell:
          key: dnd2024:spells/greater_restoration
        action: bonus_action
        save_dc: user.spell_save_dc
        level: 5
      uses:
        maximum: 1
        recovery_event: time.dawn
      usage_pool: per_bead
    - roll:
      - 17
      - 18
      name: smiting
      cast:
        spell:
          key: dnd2024:spells/shining_smite
        action: bonus_action
        save_dc: user.spell_save_dc
        level: 2
      uses:
        maximum: 1
        recovery_event: time.dawn
      usage_pool: per_bead
    - roll:
      - 19
      - 19
      name: summons
      cast:
        spell:
          key: dnd2024:spells/guardian_of_faith
        action: bonus_action
        save_dc: user.spell_save_dc
        level: 4
      uses:
        maximum: 1
        recovery_event: time.dawn
      usage_pool: per_bead
    - roll:
      - 20
      - 20
      name: wind_walking
      cast:
        spell:
          key: dnd2024:spells/wind_walk
        action: bonus_action
        save_dc: user.spell_save_dc
        level: 6
      uses:
        maximum: 1
        recovery_event: time.dawn
      usage_pool: per_bead
    attunement_required: true
splat_contracts:
- id: dnd2024:item_necklace_of_prayer_beads
  inputs:
    actor:
      type: participant
      required: true
    item:
      type: item_instance
      required: true
    targets:
      required: false
      type: participant_list
    objects:
      required: false
      type: object_list
    destination:
      required: false
      type: location
    selection:
      required: false
      type: choice
    context:
      required: false
      type: resolution
  outputs:
  - effects
  - item_state_changes
  events:
  - spell.cast_pending
  - item.contents_changed
  - time.dawn
  - item.attunement_completed
  - item.attunement_ended
  capabilities:
  - item_effects
  - dice
  - item_spellcasting
  - summoning
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: e901c21c8af260c4f780695b5013a50011fdeb9a5cbb0476498073fe01d0dbfb
  source_path: builtin/srd-5.2.1/magic_item/necklace_of_prayer_beads.md
  pages:
  - 233
  engine_gaps: &id001
  - dnd2024:item_necklace_of_prayer_beads
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_3.md
  specification_sha256: 4ccd516422bc9001b0fe259a54f83cd36198aa4e06634274b4260d4b72a7d5c8
splat_clauses:
- key: source_rules
  text: '# Necklace of Prayer Beads Wondrous Item, Rare (Requires Attunement by a Cleric, Druid, or Paladin)
    This necklace has 1d4 + 2 magic beads made from aquamarine, black pearl, or topaz. It also has many
    nonmagical beads made from stones such as amber, bloodstone, citrine, coral, jade, pearl, or quartz.
    If a magic bead is removed from the necklace, that bead loses its magic. Six types of magic beads
    exist. The GM decides the type of each bead on the necklace or determines it randomly by rolling on
    the table below. A necklace can have more than one bead of the same type. To use one, you must be
    wearing the necklace. Each bead contains a spell that you can cast from it as a Bonus Action (using
    your spell save DC if a save is necessary). Once a magic bead’s spell is cast, that bead can’t be
    used again until the next dawn. 1d20 Bead Spell 1–6 Bead of Blessing Bless 7–12 Bead of Curing Cure
    Wounds (level 2 version) 13–16 Bead of Favor Greater Restoration 17–18 Bead of Smiting Shining Smite
    19 Bead of Summons Guardian of Faith 20 Bead of Wind Wind Walk Walking'
  sha256: 69301279ccf4ee6ed26ca3413ae9db0b0799f7be5447da48aa66ca14f23c7559
  status: contract_specified
splat_requires_mechanics: *id001
---

# Necklace of Prayer Beads

<!-- SRD page 233 -->

Wondrous Item, Rare (Requires Attunement by a\
Cleric, Druid, or Paladin)\
This necklace has 1d4 + 2 magic beads made from\
aquamarine, black pearl, or topaz. It also has many\
nonmagical beads made from stones such as amber,\
bloodstone, citrine, coral, jade, pearl, or quartz. If a\
magic bead is removed from the necklace, that bead\
loses its magic.\
Six types of magic beads exist. The GM decides the\
type of each bead on the necklace or determines it\
randomly by rolling on the table below. A necklace\
can have more than one bead of the same type. To\
use one, you must be wearing the necklace. Each\
bead contains a spell that you can cast from it as a\
Bonus Action (using your spell save DC if a save is\
necessary). Once a magic bead’s spell is cast, that\
bead can’t be used again until the next dawn.\
1d20 Bead Spell\
1–6 Bead of Blessing Bless\
7–12 Bead of Curing Cure Wounds (level 2\
version)\
13–16 Bead of Favor Greater Restoration\
17–18 Bead of Smiting Shining Smite\
19 Bead of Summons Guardian of Faith\
20 Bead of Wind Wind Walk\
Walking\

Original layout: [[../../srd-5.2.1/pages/233|page 233]].
