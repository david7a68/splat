---
splat_schema: 2
splat_id: acb90637-d08a-534b-a064-87f0cc446e80
splat_kind: item
splat_key: dnd2024:items/dwarven_thrower
splat_name: Dwarven Thrower
splat_origin: dnd2024:reference/magic_item/dwarven_thrower
splat_sources:
- SRD 5.2.1, pp. 219, https://www.dndbeyond.com/srd
---

# Dwarven Thrower


## Item

| Field | Value |
| --- | --- |
| Type | Weapon |
| Rarity | Very Rare |
| Attunement | Yes |


## Source text

<!-- SRD page 219 -->

Weapon (Warhammer), Very Rare (Requires Attunement by a Dwarf or a Creature Attuned to a Belt of Dwarvenkind)

You gain a +3 bonus to attack rolls and damage rolls made with this magic weapon. It has the Thrown property with a normal range of 20 feet and a long range of 60 feet. When you hit with a ranged attack using this weapon, it deals an extra 1d8 Force damage, or an extra 2d8 Force damage if the target is a Giant. Immediately after hitting or missing, the weapon flies back to your hand.

Original layout: [[../../srd-5.2.1/pages/219|page 219]].





## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/dwarven_thrower ` |
| ` implementation ` | ` dnd2024:item_dwarven_thrower ` |

### Parameters

#### Weapon bonus

<!-- splat:values path=splat_rule/parameters/weapon_bonus -->

| Field | Value |
| --- | --- |
| ` attack ` | ` 3 ` |
| ` damage ` | ` 3 ` |

#### Attunement eligibility

##### Any of

<!-- splat:table
path: splat_rule/parameters/attunement_eligibility/any_of
key: Key
order:
- row_1
- row_2
columns:
  Key:
    type: key
  Species:
    path: species
    type: symbol
    optional: true
  Attuned item:
    path: attuned_item
    type: reference
    optional: true
-->

| Key | Species | Attuned item |
| --- | --- | --- |
| ` row_1 ` | ` Dwarf ` |  |
| ` row_2 ` |  | [Belt of Dwarvenkind](<belt_of_dwarvenkind.md> "dnd2024:items/belt_of_dwarvenkind") |

#### Thrown

<!-- splat:values path=splat_rule/parameters/thrown -->

| Field | Value |
| --- | --- |
| ` normal_range_ft ` | ` 20 ` |
| ` long_range_ft ` | ` 60 ` |

#### Ranged hit extra damage

<!-- splat:yaml path=splat_rule/parameters/ranged_hit_extra_damage -->

```yaml
type: force
normal:
  dice: 1
  sides: 8
  bonus: 0
giant:
  dice: 2
  sides: 8
  bonus: 0
```

#### Return to hand

<!-- splat:values path=splat_rule/parameters/return_to_hand -->

| Field | Value |
| --- | --- |
| ` event ` | ` ranged_attack_hit_or_miss ` |
| ` timing ` | ` immediate ` |

#### Attunement

<!-- splat:values path=splat_rule/parameters/attunement -->

| Field | Value |
| --- | --- |
| ` required ` | ` true ` |

##### Eligibility

###### Any of

<!-- splat:table
path: splat_rule/parameters/attunement/eligibility/any_of
key: Key
order:
- row_1
- row_2
columns:
  Key:
    type: key
  Species:
    path: species
    type: symbol
    optional: true
  Attuned item:
    path: attuned_item
    type: reference
    optional: true
-->

| Key | Species | Attuned item |
| --- | --- | --- |
| ` row_1 ` | ` Dwarf ` |  |
| ` row_2 ` |  | [Belt of Dwarvenkind](<belt_of_dwarvenkind.md> "dnd2024:items/belt_of_dwarvenkind") |

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_item_extras -->
> 
> ```yaml
> equipment:
>   attunement_requirement_text: a Dwarf or a Creature Attuned to a Belt of Dwarvenkind
> ```

