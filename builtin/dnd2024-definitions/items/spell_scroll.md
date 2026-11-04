---
splat_schema: 2
splat_id: 6c43d977-5f9f-588b-98c0-de2cfe98c2dc
splat_kind: item
splat_key: dnd2024:items/spell_scroll
splat_name: Spell Scroll
splat_origin: dnd2024:reference/magic_item/spell_scroll
splat_sources:
- SRD 5.2.1, pp. 244, https://www.dndbeyond.com/srd
---

# Spell Scroll


## Item

| Field | Value |
| --- | --- |
| Type | Scroll |
| Rarity | Rarity Varies |
| Attunement | No |


## Source text

<!-- SRD page 244 -->

Scroll, Rarity Varies

A Spell Scroll bears the words of a single spell, written in a mystical cipher. If the spell is on your spell list, you can read the scroll and cast its spell without Material components. Otherwise, the scroll is unintelligible. Casting the spell by reading the scroll requires the spell’s normal casting time. Once the spell is cast, the scroll crumbles to dust. If the casting is interrupted, the scroll isn’t lost.

If the spell is on your spell list but of a higher level than you can normally cast, you make an ability check using your spellcasting ability to determine whether you cast the spell. The DC equals 10 plus the spell’s level. On a failed check, the spell disappears from the scroll with no other effect.

The level of the spell on the scroll determines the spell’s saving throw DC and attack bonus, as well as the scroll’s rarity, as shown in the following table.

| Spell Level | Rarity | Save DC | Attack Bonus |
| --- | --- | --- | --- |
| Cantrip | Common | 13 | +5 |
| 1 | Common | 13 | +5 |
| 2 | Uncommon | 13 | +5 |
| 3 | Uncommon | 15 | +7 |

| Spell Level | Rarity | Save DC | Attack Bonus |
| --- | --- | --- | --- |
| 4 | Rare | 15 | +7 |
| 5 | Rare | 17 | +9 |
| 6 | Very Rare | 17 | +9 |
| 7 | Very Rare | 18 | +10 |
| 8 | Very Rare | 18 | +10 |
| 9 | Legendary | 19 | +11 |

Copying a Scroll into a Spellbook. A Wizard spell on a Spell Scroll can be copied into a spellbook. When a spell is copied in this way, the copier must succeed on an Intelligence (Arcana) check with a DC equal to 10 plus the spell’s level. On a successful check, the spell is copied. Whether the check succeeds or fails, the Spell Scroll is destroyed.

Original layout: [[../../srd-5.2.1/pages/244|page 244]].





## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/spell_scroll ` |
| ` implementation ` | ` dnd2024:item_spell_scroll ` |

### Parameters

#### Stored spell

<!-- splat:values path=splat_rule/parameters/stored_spell -->

| Field | Value |
| --- | --- |
| ` count ` | ` 1 ` |
| ` definition_type ` | ` spell ` |

#### Read and cast

<!-- splat:yaml path=splat_rule/parameters/read_and_cast -->

```yaml
requires: spell_on_user_spell_list
otherwise: unintelligible
casting_time: normal_spell_casting_time
material_components_required: false
on_cast_completed: destroy_scroll
on_cast_interrupted: preserve_scroll
higher_than_normally_castable:
  check:
    ability: user_spellcasting_ability
    dc:
      sum:
      - 10
      - value_from: spell_level
  failure:
  - erase_spell
  - no_other_effect
```

#### Level table

<!-- splat:table
path: splat_rule/parameters/level_table
key: Key
order:
- row_1
- row_2
- row_3
- row_4
- row_5
- row_6
- row_7
- row_8
- row_9
- row_10
columns:
  Key:
    type: key
  Spell level:
    path: spell_level
    type: integer
  Rarity:
    path: rarity
    type: symbol
  Save dc:
    path: save_dc
    type: integer
  Attack bonus:
    path: attack_bonus
    type: integer
-->

| Key | Spell level | Rarity | Save dc | Attack bonus |
| --- | --- | --- | --- | --- |
| ` row_1 ` | ` 0 ` | ` Common ` | ` 13 ` | ` 5 ` |
| ` row_2 ` | ` 1 ` | ` Common ` | ` 13 ` | ` 5 ` |
| ` row_3 ` | ` 2 ` | ` Uncommon ` | ` 13 ` | ` 5 ` |
| ` row_4 ` | ` 3 ` | ` Uncommon ` | ` 15 ` | ` 7 ` |
| ` row_5 ` | ` 4 ` | ` Rare ` | ` 15 ` | ` 7 ` |
| ` row_6 ` | ` 5 ` | ` Rare ` | ` 17 ` | ` 9 ` |
| ` row_7 ` | ` 6 ` | ` Very rare ` | ` 17 ` | ` 9 ` |
| ` row_8 ` | ` 7 ` | ` Very rare ` | ` 18 ` | ` 10 ` |
| ` row_9 ` | ` 8 ` | ` Very rare ` | ` 18 ` | ` 10 ` |
| ` row_10 ` | ` 9 ` | ` Legendary ` | ` 19 ` | ` 11 ` |

#### Copy to spellbook

<!-- splat:yaml path=splat_rule/parameters/copy_to_spellbook -->

```yaml
requires: wizard_spell
check:
  ability: intelligence
  skill: arcana
  dc:
    sum:
    - 10
    - value_from: spell_level
success: copy_spell
regardless_of_result: destroy_scroll
```

#### Attunement

<!-- splat:values path=splat_rule/parameters/attunement -->

| Field | Value |
| --- | --- |
| ` required ` | ` false ` |
