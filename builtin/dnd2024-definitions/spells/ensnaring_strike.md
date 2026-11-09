---
splat_schema: 2
splat_id: 5b60481a-9528-52d3-ab33-74a1060eca47
splat_kind: spell
splat_key: dnd2024:spells/ensnaring_strike
splat_name: Ensnaring Strike
splat_origin: dnd2024:reference/spell/ensnaring_strike
splat_sources:
- SRD 5.2.1, pp. 128, https://www.dndbeyond.com/srd
---

# Ensnaring Strike


## Spell

| Field | Value |
| --- | --- |
| Level | 1 |
| School | Conjuration |
| Classes | Ranger |
| Casting Time | Bonus Action, which you take immediately after hitting a creature with a weapon |
| Range |  |
| Components | V |
| Duration | Concentration, up to 1 minute |
| Ritual | No |
| Concentration | Yes |


## Source text

<!-- SRD page 128 -->

Level 1 Conjuration (Ranger)\
Casting Time: Bonus Action, which you take immediately after hitting a creature with a weapon\
Range: Self\
Components: V\
Duration: Concentration, up to 1 minute

As you hit the target, grasping vines appear on it, and it makes a Strength saving throw. A Large or larger creature has Advantage on this save. On a failed save, the target has the Restrained condition until the spell ends. On a successful save, the vines shrivel away, and the spell ends.

While Restrained, the target takes 1d6 Piercing damage at the start of each of its turns. The target or a creature within reach of it can take an action to make a Strength (Athletics) check against your spell save DC. On a success, the spell ends.

Using a Higher-Level Spell Slot. The damage increases by 1d6 for each spell slot level above 1.

Original layout: [[../../srd-5.2.1/pages/128|page 128]].





## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/ensnaring_strike ` |
| ` implementation ` | ` dnd2024:spell_ensnaring_strike ` |

### Parameters

#### Targeting

<!-- splat:values path=splat_rule/parameters/targeting -->

| Field | Value |
| --- | --- |
| ` kind ` | ` creature_hit_by_triggering_weapon_attack ` |

#### Save

<!-- splat:yaml path=splat_rule/parameters/save -->

```yaml
ability: strength
advantage_if_size_at_least: large
failure:
  condition: restrained
  until: spell_ends
success:
  end_spell: true
```

#### Restrained damage

<!-- splat:yaml path=splat_rule/parameters/restrained_damage -->

```yaml
event: target.turn_started
damage:
  dice: 1d6
  type: piercing
```

#### Escape

<!-- splat:yaml path=splat_rule/parameters/escape -->

```yaml
actor: target_or_creature_within_reach
economy: action
check:
  ability: strength
  skill: athletics
  dc: spell_save_dc
success: end_spell
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
concentration: true
maximum:
  value: 1
  unit: minute
```

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_spell_extras -->
> 
> ```yaml
> range:
>   mode: self
> ```

