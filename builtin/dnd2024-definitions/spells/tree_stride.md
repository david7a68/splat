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
---

# Tree Stride

## Source text

<!-- SRD page 170 -->

Level 5 Conjuration (Druid, Ranger)\
Casting Time: Action\
Range: Self\
Components: V, S\
Duration: Concentration, up to 1 minute

You gain the ability to enter a tree and move from inside it to inside another tree of the same kind within 500 feet. Both trees must be living and at least the same size as you. You must use 5 feet of movement to enter a tree. You instantly know the location of all other trees of the same kind within 500 feet and, as part of the move used to enter the tree, can either pass into one of those trees or step out of the tree you’re in. You appear in a spot of your choice within 5 feet of the destination tree, using another 5 feet of movement. If you have no movement left, you appear within 5 feet of the tree you entered.

You can use this transportation ability only once on each of your turns. You must end each turn outside a tree.

Original layout: [[../../srd-5.2.1/pages/170|page 170]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 5 ` |
| ` school ` | ` conjuration ` |
| ` classes ` | ` ["druid", "ranger"] ` |
| ` components ` | ` ["verbal", "somatic"] ` |
| ` concentration ` | ` true ` |
| ` duration ` | ` Concentration, up to 1 minute ` |
| ` casting_time ` | ` Action ` |
| ` ritual ` | ` false ` |

### Range

<!-- splat:values path=splat_spell/range -->

| Field | Value |
| --- | --- |
| ` mode ` | ` self ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/tree_stride ` |
| ` implementation ` | ` dnd2024:spell_tree_stride ` |

### Parameters

#### Transport

<!-- splat:yaml path=splat_rule/parameters/transport -->

```yaml
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
```
