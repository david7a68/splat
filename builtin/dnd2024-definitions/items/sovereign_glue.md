---
splat_schema: 2
splat_id: ee140b38-12f7-5951-bda1-66881e7118fc
splat_kind: item
splat_key: dnd2024:items/sovereign_glue
splat_name: Sovereign Glue
splat_origin: dnd2024:reference/magic_item/sovereign_glue
splat_sources:
- SRD 5.2.1, pp. 244, https://www.dndbeyond.com/srd
---

# Sovereign Glue


## Item

| Field | Value |
| --- | --- |
| Type | Wondrous Item |
| Rarity | Legendary |
| Attunement | No |


## Source text

<!-- SRD page 244 -->

Wondrous Item, Legendary

This viscous, milky-white substance can form a permanent adhesive bond between any two objects. It must be stored in a jar or flask that has been coated inside with Oil of Slipperiness. When found, a container contains 1d6 + 1 ounces.

One ounce of the glue can cover a 1-foot square surface. Applying an ounce of Sovereign Glue takes a Utilize action, and the applied glue takes 1 minute to set. Once it has done so, the bond it creates can be broken only by the application of Universal Solvent or Oil of Etherealness, or with a Wish spell.

Original layout: [[../../srd-5.2.1/pages/244|page 244]].





## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/sovereign_glue ` |
| ` implementation ` | ` dnd2024:item_sovereign_glue ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` attunement_required ` | ` false ` |

#### Initial ounces

<!-- splat:yaml path=splat_rule/parameters/initial_ounces -->

```yaml
roll:
  count: 1
  sides: 6
  add: 1
```

#### Storage

<!-- splat:yaml path=splat_rule/parameters/storage -->

```yaml
container_types:
- jar
- flask
interior_coating:
  key: dnd2024:items/oil_of_slipperiness
```

#### Apply

<!-- splat:yaml path=splat_rule/parameters/apply -->

```yaml
action: utilize
ounces_per_action: 1
square_ft_per_ounce: 1
bond_targets: two_objects
set_time:
  amount: 1
  unit: minutes
consumes_applied_ounces: true
```

#### Bond

<!-- splat:yaml path=splat_rule/parameters/bond -->

```yaml
permanent: true
broken_only_by:
- key: dnd2024:items/universal_solvent
- key: dnd2024:items/oil_of_etherealness
- key: dnd2024:spells/wish
```
