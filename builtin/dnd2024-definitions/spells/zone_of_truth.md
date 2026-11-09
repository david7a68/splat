---
splat_schema: 2
splat_id: 93abf320-7330-5202-b16c-a1d5dfee1f4c
splat_kind: spell
splat_key: dnd2024:spells/zone_of_truth
splat_name: Zone of Truth
splat_origin: dnd2024:reference/spell/zone_of_truth
splat_sources:
- SRD 5.2.1, pp. 175, https://www.dndbeyond.com/srd
---

# Zone of Truth


## Spell

| Field | Value |
| --- | --- |
| Level | 2 |
| School | Enchantment |
| Classes | Bard, Cleric, Paladin |
| Casting Time | Action |
| Range | 60 feet |
| Components | V, S |
| Duration | 10 minutes |
| Ritual | No |
| Concentration | No |


## Source text

<!-- SRD page 175 -->

Level 2 Enchantment (Bard, Cleric, Paladin)\
Casting Time: Action\
Range: 60 feet\
Components: V, S\
Duration: 10 minutes

You create a magical zone that guards against deception in a 15-foot-radius Sphere centered on a point within range. Until the spell ends, a creature that enters the spell’s area for the first time on a turn or starts its turn there makes a Charisma saving throw. On a failed save, a creature can’t speak a deliberate lie while in the radius. You know whether a creature succeeds or fails on this save.

An affected creature is aware of the spell and can avoid answering questions to which it would normally respond with a lie. Such a creature can be evasive yet must be truthful.

Original layout: [[../../srd-5.2.1/pages/175|page 175]].





## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/zone_of_truth ` |
| ` implementation ` | ` dnd2024:spell_zone_of_truth ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` save_triggers ` | ` ["creature_enters_area_first_time_on_turn", "creature_starts_turn_in_area"] ` |
| ` caster_knows_each_save_outcome ` | ` true ` |

#### Area

<!-- splat:values path=splat_rule/parameters/area -->

| Field | Value |
| --- | --- |
| ` shape ` | ` sphere ` |
| ` radius_feet ` | ` 15 ` |
| ` origin ` | ` chosen_point_within_range ` |
| ` duration ` | ` spell_duration ` |

#### Save

<!-- splat:yaml path=splat_rule/parameters/save -->

```yaml
ability: charisma
dc:
  binding: save_dc
failure:
  cannot_speak_deliberate_lie: true
  restriction_only_in_radius: true
  aware_of_spell: true
  may_refuse_answer: true
  may_evade_truthfully: true
success:
  effect: none
```
