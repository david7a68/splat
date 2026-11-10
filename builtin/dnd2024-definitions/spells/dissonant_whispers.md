---
splat_schema: 2
splat_id: 92291c5c-5ac9-5558-b0d0-d4b9c99f383c
splat_kind: spell
splat_key: dnd2024:spells/dissonant_whispers
splat_name: Dissonant Whispers
splat_origin: dnd2024:reference/spell/dissonant_whispers
splat_sources:
- SRD 5.2.1, pp. 124, https://www.dndbeyond.com/srd
---

# Dissonant Whispers


## Spell

| Field | Value |
| --- | --- |
| Level | 1 |
| School | Enchantment |
| Classes | Bard |
| Casting Time | Action |
| Range | 60 feet |
| Components | V |
| Duration | Instantaneous |
| Ritual | No |
| Concentration | No |


## Source text

<!-- SRD page 124 -->

Level 1 Enchantment (Bard)\
Casting Time: Action\
Range: 60 feet\
Components: V\
Duration: Instantaneous

One creature of your choice that you can see within range hears a discordant melody in its mind. The target makes a Wisdom saving throw. On a failed save, it takes 3d6 Psychic damage and must immediately use its Reaction, if available, to move as far away from you as it can, using the safest route. On a successful save, the target takes half as much damage only.

Using a Higher-Level Spell Slot. The damage increases by 1d6 for each spell slot level above 1.

Original layout: [[../../srd-5.2.1/pages/124|page 124]].





## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/dissonant_whispers ` |
| ` implementation ` | ` dnd2024:spell_dissonant_whispers ` |

### Parameters

#### Targeting

<!-- splat:values path=splat_rule/parameters/targeting -->

| Field | Value |
| --- | --- |
| ` kind ` | ` creature ` |
| ` count ` | ` 1 ` |
| ` visible ` | ` true ` |
| ` within_range ` | ` true ` |

#### Save

<!-- splat:yaml path=splat_rule/parameters/save -->

```yaml
ability: wisdom
failure:
  damage:
    dice: 3d6
    type: psychic
  movement:
    economy: reaction_if_available
    timing: immediate
    direction: away_from_caster
    distance: maximum_possible
    route: safest
success:
  damage_fraction:
    numerator: 1
    denominator: 2
  movement: false
```

#### Slot scaling

<!-- splat:values path=splat_rule/parameters/slot_scaling -->

| Field | Value |
| --- | --- |
| ` above ` | ` 1 ` |
| ` damage_per_level ` | ` 1d6 ` |

#### Spell lifetime

<!-- splat:yaml path=splat_rule/parameters/spell_lifetime -->

```yaml
concentration: false
maximum:
  kind: instantaneous
```
