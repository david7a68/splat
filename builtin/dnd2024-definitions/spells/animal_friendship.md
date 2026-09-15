---
splat_schema: 1
splat_id: d4c2380e-ac78-5579-8af5-fd603381dca7
splat_kind: spell
splat_key: dnd2024:spells/animal_friendship
splat_name: Animal Friendship
splat_origin:
  key: dnd2024:reference/spell/animal_friendship
splat_sources:
- title: SRD 5.2.1
  locator: pp. 107, 108
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 1
  school: enchantment
  classes:
  - bard
  - druid
  - ranger
  components:
  - verbal
  - somatic
  - material
  concentration: false
  duration: 24 hours
  casting_time: Action
  ritual: false
  range:
    feet: 30
  material:
    description: a morsel of food
    consumed: false
splat_rule:
  mechanic: dnd2024:spells/animal_friendship
  implementation: dnd2024:spell_animal_friendship
  parameters:
    targeting:
      kind: creature
      creature_types:
      - beast
      visible: true
      count:
        base: 1
        per_slot_above: 1
        slot_base: 1
    save:
      ability: wisdom
      failure:
        condition: charmed
        source: caster
        duration: spell_duration
    termination:
      event: damage.dealt
      dealer: caster_or_caster_ally
      recipient: target
      scope: spell
    spell_lifetime:
      concentration: false
      maximum:
        value: 24
        unit: hour
splat_contracts:
- id: dnd2024:spell_animal_friendship
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
  outputs:
  - effects
  - damage_results
  events:
  - spell.cast_completed
  - damage.applied
  - effect.ended
  capabilities:
  - spellcasting
  - effects
  - damage
  - conditions
splat_conversion:
  status: specified
  category: spell
  source_sha256: 84a51625715c6909816f4784eb739427d177eacf0f1a0cb019287b03895e40fc
  source_path: builtin/srd-5.2.1/spell/animal_friendship.md
  pages:
  - 107
  - 108
  engine_gaps: &id001
  - dnd2024:spell_animal_friendship
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_2.md
  specification_sha256: 4733244710767cd7583bb3253801b6110cceab39bc1f9769595040db95399aa9
splat_clauses:
- key: source_rules
  text: '# Animal Friendship Level 1 Enchantment (Bard, Druid, Ranger) Casting Time: Action Range: 30
    feet Components: V, S, M (a morsel of food) Duration: 24 hours Target a Beast that you can see within
    range. The target must succeed on a Wisdom saving throw or have the Charmed condition for the duration.
    If you or one of your allies deals damage to the target, the spells ends. Using a Higher-Level Spell
    Slot. You can target one additional Beast for each spell slot level above 1.'
  sha256: 292f10b758a5cc521e3ad1141234d0df40f90042d864f8ed6403aab5c37505a3
  status: contract_specified
splat_requires_mechanics: *id001
---

# Animal Friendship

<!-- SRD page 107 -->

Level 1 Enchantment (Bard, Druid, Ranger)\
Casting Time: Action\
Range: 30 feet\
Components: V, S, M (a morsel of food)\
Duration: 24 hours\
Target a Beast that you can see within range. The\
target must succeed on a Wisdom saving throw or\
have the Charmed condition for the duration. If you\

<!-- SRD page 108 -->

or one of your allies deals damage to the target, the\
spells ends.\
Using a Higher-Level Spell Slot. You can target\
one additional Beast for each spell slot level above 1.\

Original layout: [[../../srd-5.2.1/pages/107|page 107]], [[../../srd-5.2.1/pages/108|page 108]].
