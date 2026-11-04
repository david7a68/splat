---
splat_schema: 2
splat_id: 0d7805ca-c716-5d95-a3c0-471974427771
splat_kind: spell
splat_key: dnd2024:spells/poison_spray
splat_name: Poison Spray
splat_origin: dnd2024:reference/spell/poison_spray
splat_sources:
- SRD 5.2.1, pp. 153, https://www.dndbeyond.com/srd
---

# Poison Spray


## Spell

| Field | Value |
| --- | --- |
| Level | Cantrip |
| School | Necromancy |
| Classes | Druid, Sorcerer, Warlock, Wizard |
| Casting Time | Action |
| Range | 30 feet |
| Components | V, S |
| Duration | Instantaneous |


## Source text

<!-- SRD page 153 -->

Necromancy Cantrip (Druid, Sorcerer, Warlock, Wizard)\
Casting Time: Action\
Range: 30 feet\
Components: V, S\
Duration: Instantaneous

You spray toxic mist at a creature within range. Make a ranged spell attack against the target. On a hit, the target takes 1d12 Poison damage.

Cantrip Upgrade. The damage increases by 1d12 when you reach levels 5 (2d12), 11 (3d12), and 17 (4d12).

Original layout: [[../../srd-5.2.1/pages/153|page 153]].





## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/poison_spray ` |
| ` implementation ` | ` dnd2024:spell_poison_spray ` |

### Parameters

#### Target

<!-- splat:values path=splat_rule/parameters/target -->

| Field | Value |
| --- | --- |
| ` kind ` | ` creature ` |
| ` count ` | ` 1 ` |
| ` visible_to_caster ` | ` false ` |

#### Attack

<!-- splat:yaml path=splat_rule/parameters/attack -->

```yaml
kind: ranged_spell
hit:
  damage:
    amount:
      dice:
        count: 1
        sides: 12
        add: 0
    type: poison
```

#### Cantrip dice by character level

<!-- splat:yaml path=splat_rule/parameters/cantrip_dice_by_character_level -->

```yaml
'1': 1
'5': 2
'11': 3
'17': 4
```
