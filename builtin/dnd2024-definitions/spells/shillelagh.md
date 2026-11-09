---
splat_schema: 2
splat_id: b80a91c5-1e35-50d3-bbba-51b10ce494d2
splat_kind: spell
splat_key: dnd2024:spells/shillelagh
splat_name: Shillelagh
splat_origin: dnd2024:reference/spell/shillelagh
splat_sources:
- SRD 5.2.1, pp. 162, https://www.dndbeyond.com/srd
---

# Shillelagh


## Spell

| Field | Value |
| --- | --- |
| Level | Cantrip |
| School | Transmutation |
| Classes | Druid |
| Casting Time | Bonus Action |
| Range |  |
| Components | V, S, M (mistletoe) |
| Duration | 1 minute |
| Ritual | No |
| Concentration | No |


## Source text

<!-- SRD page 162 -->

Transmutation Cantrip (Druid)\
Casting Time: Bonus Action\
Range: Self\
Components: V, S, M (mistletoe)\
Duration: 1 minute

A Club or Quarterstaff you are holding is imbued with nature’s power. For the duration, you can use your spellcasting ability instead of Strength for the attack and damage rolls of melee attacks using that weapon, and the weapon’s damage die becomes a d8. If the attack deals damage, it can be Force damage or the weapon’s normal damage type (your choice).

The spell ends early if you cast it again or if you let go of the weapon.

Cantrip Upgrade. The damage die changes when you reach levels 5 (d10), 11 (d12), and 17 (2d6).

Original layout: [[../../srd-5.2.1/pages/162|page 162]].







## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/shillelagh ` |
| ` implementation ` | ` dnd2024:spell_shillelagh ` |

### Parameters

#### Target

<!-- splat:values path=splat_rule/parameters/target -->

| Field | Value |
| --- | --- |
| ` kind ` | ` held_weapon ` |
| ` allowed ` | ` ["club", "quarterstaff"] ` |

#### Melee attacks

<!-- splat:yaml path=splat_rule/parameters/melee_attacks -->

```yaml
optional_ability_replacement: spellcasting_instead_of_strength
applies_to:
- attack_roll
- damage_roll
damage_dice_by_character_level:
  '1':
    dice:
      count: 1
      sides: 8
      add: 0
  '5':
    dice:
      count: 1
      sides: 10
      add: 0
  '11':
    dice:
      count: 1
      sides: 12
      add: 0
  '17':
    dice:
      count: 2
      sides: 6
      add: 0
damage_type_choice:
- force
- weapon_normal
```

#### End

<!-- splat:values path=splat_rule/parameters/end -->

| Field | Value |
| --- | --- |
| ` events ` | ` ["caster.recasts_same_spell", "caster.lets_go_of_weapon"] ` |

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_spell_extras -->
> 
> ```yaml
> material:
>   consumed: false
> range:
>   mode: self
> ```

