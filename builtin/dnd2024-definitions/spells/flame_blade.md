---
splat_schema: 1
splat_id: 3c2661a3-d5fe-56e7-9c6e-8e5439258de8
splat_kind: spell
splat_key: dnd2024:spells/flame_blade
splat_name: Flame Blade
splat_origin:
  key: dnd2024:reference/spell/flame_blade
splat_sources:
- title: SRD 5.2.1
  locator: pp. 132
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 2
  school: evocation
  classes:
  - druid
  - sorcerer
  components:
  - verbal
  - somatic
  - material
  concentration: true
  duration: Concentration, up to 10 minutes
  casting_time: Bonus Action
  ritual: false
  range:
    mode: self
  material:
    description: a sumac leaf
    consumed: false
splat_rule:
  mechanic: dnd2024:spells/flame_blade
  implementation: dnd2024:spell_flame_blade
  parameters:
    creation:
      requires_free_hand: true
      form: scimitar_sized_shaped_blade
      duration: spell_duration
      release: disappear
      reevoke_cost: bonus_action
      light:
        bright_radius_feet: 10
        additional_dim_feet: 10
    attack:
      cost: magic_action
      kind: melee_spell
      hit:
        dice:
          count:
            op: add
            args:
            - 3
            - op: multiply
              args:
              - 1
              - op: subtract
                args:
                - binding: cast_level
                - 2
          sides: 6
          add:
            binding: spellcasting_modifier
        type: fire
splat_contracts:
- id: dnd2024:spell_flame_blade
  inputs:
    caster:
      type: participant
      required: true
    cast_level:
      type: integer
      required: true
    spellcasting_modifier:
      type: integer
      required: true
    choices:
      required: false
      type: choice
    items:
      required: false
      type: item_list
  outputs:
  - effects
  - attacks
  - items
  events:
  - spell.cast_completed
  - effect.ended
  capabilities:
  - spellcasting
  - effects
  - dice
  - attacks
  - inventory
splat_conversion:
  status: specified
  category: spell
  source_sha256: c4a5f260527a5ad42b0b4c24ad5734becd8ae21b0cfba58ce6f4f3ccb9641476
  source_path: builtin/srd-5.2.1/spell/flame_blade.md
  pages:
  - 132
  engine_gaps: &id001
  - dnd2024:spell_flame_blade
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_3.md
  specification_sha256: 823bc7e41d5b70937124b2e9a5ed63e163a7ff4704d4da48271bd566e6a52ce9
splat_clauses:
- key: source_rules
  text: '# Flame Blade Level 2 Evocation (Druid, Sorcerer) Casting Time: Bonus Action Range: Self Components:
    V, S, M (a sumac leaf) Duration: Concentration, up to 10 minutes You evoke a fiery blade in your free
    hand. The blade is similar in size and shape to a scimitar, and it lasts for the duration. If you
    let go of the blade, it disappears, but you can evoke it again as a Bonus Action. As a Magic action,
    you can make a melee spell attack with the fiery blade. On a hit, the target takes Fire damage equal
    to 3d6 plus your spellcasting ability modifier. The flaming blade sheds Bright Light in a 10-foot
    radius and Dim Light for an additional 10 feet. Using a Higher-Level Spell Slot. The damage increases
    by 1d6 for each spell slot level above 2.'
  sha256: 5835b742b83e01ae30879cb9e715332e960c410d3ab487cf08cdd4ad5fe86686
  status: contract_specified
splat_requires_mechanics: *id001
---

# Flame Blade

<!-- SRD page 132 -->

Level 2 Evocation (Druid, Sorcerer)\
Casting Time: Bonus Action\
Range: Self\
Components: V, S, M (a sumac leaf)\
Duration: Concentration, up to 10 minutes\
You evoke a fiery blade in your free hand. The blade\
is similar in size and shape to a scimitar, and it lasts\
for the duration. If you let go of the blade, it disap-\
pears, but you can evoke it again as a Bonus Action.\
As a Magic action, you can make a melee spell at-\
tack with the fiery blade. On a hit, the target takes\
Fire damage equal to 3d6 plus your spellcasting\
ability modifier.\
The flaming blade sheds Bright Light in a 10-foot\
radius and Dim Light for an additional 10 feet.\
Using a Higher-Level Spell Slot. The damage in-\
creases by 1d6 for each spell slot level above 2.\

Original layout: [[../../srd-5.2.1/pages/132|page 132]].
