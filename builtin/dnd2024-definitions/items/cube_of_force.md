---
splat_schema: 1
splat_id: 3d74ea97-d180-5a06-881c-f9d30d23f1ad
splat_kind: item
splat_key: dnd2024:items/cube_of_force
splat_name: Cube of Force
splat_origin:
  key: dnd2024:reference/magic_item/cube_of_force
splat_sources:
- title: SRD 5.2.1
  locator: pp. 216
  url: https://www.dndbeyond.com/srd
---

# Cube of Force

## Source text

<!-- SRD page 216 -->

Wondrous Item, Rare (Requires Attunement)

This cube is about an inch across. Each face has a distinct marking on it. You can press one of those faces, expend the number of charges required for it, and thereby cast the spell associated with it (save DC 17), as shown in the Cube of Force Faces table.

The cube starts with 10 charges, and it regains 1d6 expended charges daily at dawn.

**Cube of Force Faces**

| Spell | Charge Cost |
| --- | --- |
| Mage Armor | 1 |
| Shield | 1 |
| Tiny Hut | 3 |
| Private Sanctum | 4 |
| Resilient Sphere | 4 |
| Wall of Force | 5 |

Original layout: [[../../srd-5.2.1/pages/216|page 216]].

## Item

<!-- splat:values path=splat_item -->

| Field | Value |
| --- | --- |
| ` category ` | ` wondrous_item ` |
| ` rarity ` | ` rare ` |

### Equipment

<!-- splat:values path=splat_item/equipment -->

| Field | Value |
| --- | --- |
| ` attunement ` | ` required ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/cube_of_force ` |
| ` implementation ` | ` dnd2024:item_cube_of_force ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` dimensions_inches ` | ` [1, 1, 1] ` |
| ` activation ` | ` press_spell_face ` |
| ` attunement_required ` | ` true ` |

#### Charges

<!-- splat:yaml path=splat_rule/parameters/charges -->

```yaml
initial: 10
maximum: 10
recovery:
  event: time.dawn
  amount:
    roll:
      count: 1
      sides: 6
  cap_at_maximum: true
```

#### Spells

<!-- splat:table
path: splat_rule/parameters/spells
key: Spell
order:
- mage_armor
- shield
- tiny_hut
- private_sanctum
- resilient_sphere
- wall_of_force
columns:
  Spell:
    path: spell
    type: reference
  Charge cost:
    path: charge_cost
    type: integer
  Save dc:
    path: save_dc
    type: integer
-->

| Spell | Charge cost | Save dc |
| --- | --- | --- |
| [Mage Armor](<../spells/mage_armor.md> "dnd2024:spells/mage_armor") | ` 1 ` | ` 17 ` |
| [Shield](<../spells/shield.md> "dnd2024:spells/shield") | ` 1 ` | ` 17 ` |
| [Tiny Hut](<../spells/tiny_hut.md> "dnd2024:spells/tiny_hut") | ` 3 ` | ` 17 ` |
| [Private Sanctum](<../spells/private_sanctum.md> "dnd2024:spells/private_sanctum") | ` 4 ` | ` 17 ` |
| [Resilient Sphere](<../spells/resilient_sphere.md> "dnd2024:spells/resilient_sphere") | ` 4 ` | ` 17 ` |
| [Wall of Force](<../spells/wall_of_force.md> "dnd2024:spells/wall_of_force") | ` 5 ` | ` 17 ` |
