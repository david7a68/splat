---
splat_schema: 2
splat_id: 1b5855ba-91c4-578c-9f95-eb7745f961ee
splat_kind: spell
splat_key: dnd2024:spells/calm_emotions
splat_name: Calm Emotions
splat_origin: dnd2024:reference/spell/calm_emotions
splat_sources:
- SRD 5.2.1, pp. 114, https://www.dndbeyond.com/srd
---

# Calm Emotions


## Spell

| Field | Value |
| --- | --- |
| Level | 2 |
| School | Enchantment |
| Classes | Bard, Cleric |
| Casting Time | Action |
| Range | 60 feet |
| Components | V, S |
| Duration | Concentration, up to 1 minute |
| Ritual | No |
| Concentration | Yes |


## Source text

<!-- SRD page 114 -->

Level 2 Enchantment (Bard, Cleric)\
Casting Time: Action\
Range: 60 feet\
Components: V, S\
Duration: Concentration, up to 1 minute

Each Humanoid in a 20-foot-radius Sphere centered on a point you choose within range must succeed on a Charisma saving throw or be affected by one of the following effects (choose for each creature):

•  The creature has Immunity to the Charmed and Frightened conditions until the spell ends. If the creature was already Charmed or Frightened, those conditions are suppressed for the duration.

•  The creature becomes Indifferent about creatures of your choice that it’s Hostile toward. This indifference ends if the target takes damage or witnesses its allies taking damage. When the spell ends, the creature’s attitude returns to normal.

Original layout: [[../../srd-5.2.1/pages/114|page 114]].





## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/calm_emotions ` |
| ` implementation ` | ` dnd2024:spell_calm_emotions ` |

### Parameters

#### Targeting

<!-- splat:values path=splat_rule/parameters/targeting -->

| Field | Value |
| --- | --- |
| ` kind ` | ` creatures_in_area ` |
| ` types ` | ` ["humanoid"] ` |
| ` origin ` | ` chosen_point_in_range ` |
| ` shape ` | ` sphere ` |
| ` radius_feet ` | ` 20 ` |

#### Save

<!-- splat:values path=splat_rule/parameters/save -->

| Field | Value |
| --- | --- |
| ` ability ` | ` charisma ` |
| ` failure_choice_per_target ` | ` ["suppress_conditions", "suppress_hostility"] ` |

#### Suppress conditions

<!-- splat:values path=splat_rule/parameters/suppress_conditions -->

| Field | Value |
| --- | --- |
| ` immune ` | ` ["charmed", "frightened"] ` |
| ` suppress_existing ` | ` ["charmed", "frightened"] ` |
| ` duration ` | ` spell_duration ` |

#### Suppress hostility

<!-- splat:values path=splat_rule/parameters/suppress_hostility -->

| Field | Value |
| --- | --- |
| ` attitude ` | ` indifferent ` |
| ` toward ` | ` caster_chosen_previously_hostile_creatures ` |
| ` ends_on ` | ` ["target_takes_damage", "target_witnesses_allies_take_damage", "spell_ends"] ` |
| ` restore_attitude ` | ` normal ` |

#### Spell lifetime

<!-- splat:yaml path=splat_rule/parameters/spell_lifetime -->

```yaml
concentration: true
maximum:
  value: 1
  unit: minute
```
