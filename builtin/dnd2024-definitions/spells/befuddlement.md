---
splat_schema: 1
splat_id: a70a357a-c4ad-54f7-b8d4-d7db71b23f5a
splat_kind: spell
splat_key: dnd2024:spells/befuddlement
splat_name: Befuddlement
splat_origin:
  key: dnd2024:reference/spell/befuddlement
splat_sources:
- title: SRD 5.2.1
  locator: pp. 112
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 8
  school: enchantment
  classes:
  - bard
  - druid
  - warlock
  - wizard
  components:
  - verbal
  - somatic
  - material
  concentration: false
  duration: Instantaneous
  casting_time: Action
  ritual: false
  range:
    feet: 150
  material:
    description: a key ring with no keys
    consumed: false
splat_rule:
  mechanic: dnd2024:spells/befuddlement
  implementation: dnd2024:spell_befuddlement
  parameters:
    targets:
      kind: creature
      maximum: 1
      within_spell_range: true
      visible: true
    save:
      ability: intelligence
      dc:
        binding: save_dc
      failure:
        damage:
          dice:
            count: 10
            sides: 12
            add: 0
          type: psychic
        prohibit:
        - cast_spells
        - magic_action
        repeat_save:
          interval_days: 30
          success: end_effect
        end_by_spells:
        - key: dnd2024:spells/greater_restoration
        - key: dnd2024:spells/heal
        - key: dnd2024:spells/wish
      success:
        damage_fraction:
          numerator: 1
          denominator: 2
        damage:
          dice:
            count: 10
            sides: 12
            add: 0
          type: psychic
splat_contracts:
- id: dnd2024:spell_befuddlement
  inputs:
    caster:
      type: participant
      required: true
    cast_level:
      type: integer
      required: true
    targets:
      type: participant_list
      required: true
    save_dc:
      type: integer
      required: true
    choices:
      required: false
      type: choice
  outputs:
  - effects
  - damage
  events:
  - spell.cast_completed
  - save.before
  - save.after
  - time.elapsed
  capabilities:
  - spellcasting
  - effects
  - dice
  - damage
  - saving_throws
splat_conversion:
  status: specified
  category: spell
  source_sha256: 53ec849fc9da7df536acf847f33c4b77e0124b5c961f5b873b230d4794989e8c
  source_path: builtin/srd-5.2.1/spell/befuddlement.md
  pages:
  - 112
  engine_gaps: &id001
  - dnd2024:spell_befuddlement
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_3.md
  specification_sha256: cc8f8b875d114e3a7545949843b379da2b163819b5943f269199af6df16c0f45
splat_clauses:
- key: source_rules
  text: '# Befuddlement Level 8 Enchantment (Bard, Druid, Warlock, Wizard) Casting Time: Action Range:
    150 feet Components: V, S, M (a key ring with no keys) Duration: Instantaneous You blast the mind
    of a creature that you can see within range. The target makes an Intelligence saving throw. On a failed
    save, the target takes 10d12 Psychic damage and can’t cast spells or take the Magic action. At the
    end of every 30 days, the target repeats the save, ending the effect on a success. The effect can
    also be ended by the Greater Restoration, Heal, or Wish spell. On a successful save, the target takes
    half as much damage only.'
  sha256: 254f4473041de58171962aedf6638d2d607031a3d7653ad90a3c75f82142287d
  status: contract_specified
splat_requires_mechanics: *id001
---

# Befuddlement

<!-- SRD page 112 -->

Level 8 Enchantment (Bard, Druid, Warlock, Wizard)\
Casting Time: Action\
Range: 150 feet\
Components: V, S, M (a key ring with no keys)\
Duration: Instantaneous\
You blast the mind of a creature that you can see\
within range. The target makes an Intelligence sav-\
ing throw.\
On a failed save, the target takes 10d12 Psychic\
damage and can’t cast spells or take the Magic ac-\
tion. At the end of every 30 days, the target repeats\
the save, ending the effect on a success. The effect\
can also be ended by the Greater Restoration, Heal,\
or Wish spell.\
On a successful save, the target takes half as much\
damage only.\

Original layout: [[../../srd-5.2.1/pages/112|page 112]].
