---
splat_schema: 2
splat_id: 289ed0e7-e108-5dd9-912f-bbf5385bdee9
splat_kind: item
splat_key: dnd2024:items/necklace_of_fireballs
splat_name: Necklace of Fireballs
splat_origin: dnd2024:reference/magic_item/necklace_of_fireballs
splat_sources:
- SRD 5.2.1, pp. 233, https://www.dndbeyond.com/srd
---

# Necklace of Fireballs


## Item

| Field | Value |
| --- | --- |
| Type | Wondrous Item |
| Rarity | Rare |
| Attunement | No |


## Source text

<!-- SRD page 233 -->

Wondrous Item, Rare

This necklace has 1d6 + 3 beads hanging from it. You can take a Magic action to detach a bead and throw it up to 60 feet away. When it reaches the end of its trajectory, the bead detonates as a level 3 Fireball (save DC 15).

You can hurl multiple beads, or even the whole necklace, at one time. When you do so, increase the damage of the Fireball by 1d6 for each bead after the first (maximum 12d6).

Original layout: [[../../srd-5.2.1/pages/233|page 233]].





## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/necklace_of_fireballs ` |
| ` implementation ` | ` dnd2024:item_necklace_of_fireballs ` |

### Parameters

#### Beads

<!-- splat:yaml path=splat_rule/parameters/beads -->

```yaml
initial:
  expression: 1d6+3
  scope: use
```

#### Activate

<!-- splat:yaml path=splat_rule/parameters/activate -->

```yaml
action: magic
throw_maximum_ft: 60
beads_expended:
  minimum: 1
  maximum: remaining
  whole_necklace_allowed: true
detonate_at: trajectory_end
spell:
  key: dnd2024:spells/fireball
spell_level: 3
save_dc: 15
damage_override:
  dice:
    count:
      op: min
      args:
      - 12
      - op: add
        args:
        - 8
        - op: subtract
          args:
          - binding: beads_expended
          - 1
    sides: 6
    add: 0
  type: fire
```

#### Attunement

<!-- splat:values path=splat_rule/parameters/attunement -->

| Field | Value |
| --- | --- |
| ` required ` | ` false ` |

#### Common rules

<!-- splat:rows path=splat_rule/parameters/common_rules -->

| Row | key |
| --- | --- |
| ` 0 ` | ` dnd2024:rules/magic_item_categories_wondrous_items ` |
| ` 1 ` | ` dnd2024:rules/activating_a_magic_item_spells_cast_from_items ` |
