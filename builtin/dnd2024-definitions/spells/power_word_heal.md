---
splat_schema: 1
splat_id: c6cccb53-4af6-55ef-af03-c177bb9950d2
splat_kind: spell
splat_key: dnd2024:spells/power_word_heal
splat_name: Power Word Heal
splat_origin:
  key: dnd2024:reference/spell/power_word_heal
splat_sources:
- title: SRD 5.2.1
  locator: pp. 154
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 9
  school: enchantment
  classes:
  - bard
  - cleric
  components:
  - verbal
  concentration: false
  duration: Instantaneous
  casting_time: Action
  ritual: false
  range:
    feet: 60
splat_rule:
  mechanic: dnd2024:spells/power_word_heal
  implementation: dnd2024:spell_power_word_heal
  parameters:
    targets:
      kind: creature
      maximum: 1
      within_spell_range: true
      visible: true
    healing:
      restore: all_hit_points
    end_conditions:
    - charmed
    - frightened
    - paralyzed
    - poisoned
    - stunned
    prone:
      stand_optional: true
      cost: target_reaction
splat_contracts:
- id: dnd2024:spell_power_word_heal
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
    choices:
      required: false
      type: choice
  outputs:
  - effects
  - health
  events:
  - spell.cast_completed
  capabilities:
  - spellcasting
  - effects
  - conditions
  - health
splat_conversion:
  status: specified
  category: spell
  source_sha256: e2ed6ce23eba4f917bfe535dcec9b324cf4a44cfcffcd6686463a07d6ed2c730
  source_path: builtin/srd-5.2.1/spell/power_word_heal.md
  pages:
  - 154
  engine_gaps: &id001
  - dnd2024:spell_power_word_heal
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_3.md
  specification_sha256: eb30d046b968c6ca0a9d2d2e63b281eb1cca473b53a53b820b0e6be5285b1ee8
splat_clauses:
- key: source_rules
  text: '# Power Word Heal Level 9 Enchantment (Bard, Cleric) Casting Time: Action Range: 60 feet Component:
    V Duration: Instantaneous A wave of healing energy washes over one creature you can see within range.
    The target regains all its Hit Points. If the creature has the Charmed, Frightened, Paralyzed, Poisoned,
    or Stunned condition, the condition ends. If the creature has the Prone condition, it can use its
    Reaction to stand up.'
  sha256: cb26dcebcb75f40e82e3d752147852bf493d379d8b8bcc3c5b18e3b5269073ca
  status: contract_specified
splat_requires_mechanics: *id001
---

# Power Word Heal

<!-- SRD page 154 -->

Level 9 Enchantment (Bard, Cleric)\
Casting Time: Action\
Range: 60 feet\
Component: V\
Duration: Instantaneous\
A wave of healing energy washes over one creature\
you can see within range. The target regains all its\
Hit Points. If the creature has the Charmed, Fright-\
ened, Paralyzed, Poisoned, or Stunned condition,\
the condition ends. If the creature has the Prone\
condition, it can use its Reaction to stand up.\

Original layout: [[../../srd-5.2.1/pages/154|page 154]].
