---
splat_schema: 1
splat_id: e744366a-c0e1-5396-a29d-2b5d69f08560
splat_kind: spell
splat_key: dnd2024:spells/tree_stride
splat_name: Tree Stride
splat_origin:
  key: dnd2024:reference/spell/tree_stride
splat_sources:
- title: SRD 5.2.1
  locator: pp. 170
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 5
  school: conjuration
  classes:
  - druid
  - ranger
  components:
  - verbal
  - somatic
  concentration: true
  duration: Concentration, up to 1 minute
  casting_time: Action
  ritual: false
  range:
    mode: self
splat_rule:
  mechanic: dnd2024:spells/tree_stride
  implementation: dnd2024:spell_tree_stride
  parameters:
    transport:
      duration: spell_duration
      source_and_destination_trees:
        living: true
        minimum_size: caster_size
        same_kind: true
      maximum_tree_distance_feet: 500
      entry_movement_cost_feet: 5
      on_entry_knowledge:
        locations: all_same_kind_trees
        radius_feet: 500
      as_part_of_entry_choices:
      - pass_to_eligible_tree
      - step_out_of_current_tree
      exit:
        destination_space_chooser: caster
        within_destination_tree_feet: 5
        movement_cost_feet: 5
      no_movement_remaining:
        appear_within_entered_tree_feet: 5
      uses_per_caster_turn: 1
      must_end_turn_outside_tree: true
splat_contracts:
- id: dnd2024:spell_tree_stride
  inputs:
    caster:
      type: participant
      required: true
    cast_level:
      type: integer
      required: true
    locations:
      required: false
      type: location_list
    campaign:
      required: false
      type: campaign
    choices:
      required: false
      type: choice
  outputs:
  - effects
  - knowledge
  events:
  - spell.cast_completed
  - movement.completed
  capabilities:
  - spellcasting
  - effects
  - spatial_relationships
  - knowledge
splat_conversion:
  status: specified
  category: spell
  source_sha256: 576c1668cef9be17a10e6ebbd5403782a7314d8882bb0f98cb9560cf398978de
  source_path: builtin/srd-5.2.1/spell/tree_stride.md
  pages:
  - 170
  engine_gaps: &id001
  - dnd2024:spell_tree_stride
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_3.md
  specification_sha256: 03818737608170b0b4b014220e7584d18b06c99dda46b84a7236da68c47dcd14
splat_clauses:
- key: source_rules
  text: '# Tree Stride Level 5 Conjuration (Druid, Ranger) Casting Time: Action Range: Self Components:
    V, S Duration: Concentration, up to 1 minute You gain the ability to enter a tree and move from inside
    it to inside another tree of the same kind within 500 feet. Both trees must be living and at least
    the same size as you. You must use 5 feet of movement to enter a tree. You instantly know the location
    of all other trees of the same kind within 500 feet and, as part of the move used to enter the tree,
    can either pass into one of those trees or step out of the tree you’re in. You appear in a spot of
    your choice within 5 feet of the destination tree, using another 5 feet of movement. If you have no
    movement left, you appear within 5 feet of the tree you entered. You can use this transportation ability
    only once on each of your turns. You must end each turn outside a tree.'
  sha256: 6ae1e0bbfe04e6b3eb24b84e50be19a62ca8bc48fbd4fde1794ba35679a0ed81
  status: contract_specified
splat_requires_mechanics: *id001
---

# Tree Stride

<!-- SRD page 170 -->

Level 5 Conjuration (Druid, Ranger)\
Casting Time: Action\
Range: Self\
Components: V, S\
Duration: Concentration, up to 1 minute\
You gain the ability to enter a tree and move from\
inside it to inside another tree of the same kind\
within 500 feet. Both trees must be living and at\
least the same size as you. You must use 5 feet of\
movement to enter a tree. You instantly know the\
location of all other trees of the same kind within\
500 feet and, as part of the move used to enter the\
tree, can either pass into one of those trees or step\
out of the tree you’re in. You appear in a spot of your\
choice within 5 feet of the destination tree, using\
another 5 feet of movement. If you have no move-\
ment left, you appear within 5 feet of the tree you\
entered.\
You can use this transportation ability only once\
on each of your turns. You must end each turn out-\
side a tree.\

Original layout: [[../../srd-5.2.1/pages/170|page 170]].
