---
splat_schema: 2
splat_id: 077d23fe-fa92-5924-8729-ba18e9d7a37f
splat_kind: spell
splat_key: dnd2024:spells/ray_of_sickness
splat_name: Ray of Sickness
splat_origin: dnd2024:reference/spell/ray_of_sickness
splat_sources:
- SRD 5.2.1, pp. 158, https://www.dndbeyond.com/srd
---

# Ray of Sickness


## Spell

| Field | Value |
| --- | --- |
| Level | 1 |
| School | Necromancy |
| Classes | Sorcerer, Wizard |
| Casting Time | Action |
| Range | 60 feet |
| Components | V, S |
| Duration | Instantaneous |
| Ritual | No |
| Concentration | No |


## Source text

<!-- SRD page 158 -->

Level 1 Necromancy (Sorcerer, Wizard)\
Casting Time: Action\
Range: 60 feet\
Components: V, S\
Duration: Instantaneous

You shoot a greenish ray at a creature within range. Make a ranged spell attack against the target. On a hit, the target takes 2d8 Poison damage and has the Poisoned condition until the end of your next turn.

Using a Higher-Level Spell Slot. The damage increases by 1d8 for each spell slot level above 1.

Original layout: [[../../srd-5.2.1/pages/158|page 158]].





## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/ray_of_sickness ` |
| ` implementation ` | ` dnd2024:spell_ray_of_sickness ` |

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
        count: 2
        sides: 8
        add: 0
    type: poison
  condition: poisoned
  condition_until: caster.next_turn.end
```

#### Scaling

<!-- splat:values path=splat_rule/parameters/scaling -->

| Field | Value |
| --- | --- |
| ` per_slot_above ` | ` 1 ` |
| ` damage_dice ` | ` 1 ` |
| ` sides ` | ` 8 ` |
