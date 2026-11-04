---
splat_schema: 2
splat_id: ecdd6fe2-79ad-5b35-9058-b2174a47ca71
splat_kind: item
splat_key: dnd2024:items/javelin_of_lightning
splat_name: Javelin of Lightning
splat_origin: dnd2024:reference/magic_item/javelin_of_lightning
splat_sources:
- SRD 5.2.1, pp. 228, https://www.dndbeyond.com/srd
---

# Javelin of Lightning


## Item

| Field | Value |
| --- | --- |
| Type | Weapon |
| Rarity | Uncommon |
| Attunement | No |


## Source text

<!-- SRD page 228 -->

Weapon (Javelin), Uncommon

Each time you make an attack roll with this magic weapon and hit, you can have it deal Lightning damage instead of Piercing damage.

Lightning Bolt. When you throw this weapon at a target no farther than 120 feet from you, you can forgo making a ranged attack roll and instead turn the weapon into a bolt of lightning. This bolt forms a 5-foot-wide Line between you and the target. The target and each other creature in the Line (excluding you) makes a DC 13 Dexterity saving throw, taking 4d6 Lightning damage on a failed save or half as much damage on a successful one. Immediately after dealing this damage, the weapon reappears in your hand. This property can’t be used again until the next dawn.

Original layout: [[../../srd-5.2.1/pages/228|page 228]].





## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/javelin_of_lightning ` |
| ` implementation ` | ` dnd2024:item_javelin_of_lightning ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` on_hit_damage_type_choice ` | ` ["piercing", "lightning"] ` |
| ` attunement_required ` | ` false ` |

#### Lightning bolt

<!-- splat:yaml path=splat_rule/parameters/lightning_bolt -->

```yaml
trigger: throw_at_target
max_target_range_ft: 120
replaces: ranged_attack_roll
area:
  shape: line
  width_ft: 5
  from: user
  to: target
targets:
- target
- all_other_creatures_in_line
exclude:
- user
save:
  ability: dexterity
  dc: 13
  failure:
    damage:
      amount:
        roll:
          count: 4
          sides: 6
      type: lightning
  success:
    damage_fraction:
    - 1
    - 2
after_damage: return_weapon_to_hand
uses:
  maximum: 1
  recovery_event: time.dawn
```
