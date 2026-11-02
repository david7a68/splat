---
splat_schema: 1
splat_id: 71bcb261-5e2d-5af7-b33f-f815e804046f
splat_kind: spell
splat_key: dnd2024:spells/magic_jar
splat_name: Magic Jar
splat_origin:
  key: dnd2024:reference/spell/magic_jar
splat_sources:
- title: SRD 5.2.1
  locator: pp. 146
  url: https://www.dndbeyond.com/srd
---

# Magic Jar

## Source text

<!-- SRD page 146 -->

Level 6 Necromancy (Wizard)\
Casting Time: 1 minute\
Range: Self\
Components: V, S, M (a gem, crystal, or reliquary worth 500+ GP)\
Duration: Until dispelled

Your body falls into a catatonic state as your soul leaves it and enters the container you used for the spell’s Material component. While your soul inhabits the container, you are aware of your surroundings as if you were in the container’s space. You can’t move or take Reactions. The only action you can take is to project your soul up to 100 feet out of the container, either returning to your living body (and ending the spell) or attempting to possess a Humanoid’s body.

You can attempt to possess any Humanoid within 100 feet of you that you can see (creatures warded by a Protection from Evil and Good or Magic Circle spell can’t be possessed). The target makes a Charisma saving throw. On a failed save, your soul enters the target’s body, and the target’s soul becomes trapped in the container. On a successful save, the target resists your efforts to possess it, and you can’t attempt to possess it again for 24 hours.

Once you possess a creature’s body, you control it. Your Hit Points, Hit Point Dice, Strength, Dexterity, Constitution, Speed, and senses are replaced by the creature’s. You otherwise keep your game statistics.

Meanwhile, the possessed creature’s soul can perceive from the container using its own senses, but it can’t move and it is Incapacitated.

While possessing a body, you can take a Magic action to return from the host body to the container if it is within 100 feet of you, returning the host creature’s soul to its body. If the host body dies while you’re in it, the creature dies, and you make a Charisma saving throw against your own spellcasting DC. On a success, you return to the container if it is within 100 feet of you. Otherwise, you die.

If the container is destroyed or the spell ends, your soul returns to your body. If your body is more than 100 feet away from you or if your body is dead, you die. If another creature’s soul is in the container when it is destroyed, the creature’s soul returns to its body if the body is alive and within 100 feet. Otherwise, that creature dies.

When the spell ends, the container is destroyed.

Original layout: [[../../srd-5.2.1/pages/146|page 146]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 6 ` |
| ` school ` | ` necromancy ` |
| ` classes ` | ` ["wizard"] ` |
| ` components ` | ` ["verbal", "somatic", "material"] ` |
| ` concentration ` | ` false ` |
| ` duration ` | ` Until dispelled ` |
| ` casting_time ` | ` 1 minute ` |
| ` ritual ` | ` false ` |

### Range

<!-- splat:values path=splat_spell/range -->

| Field | Value |
| --- | --- |
| ` mode ` | ` self ` |

### Material

<!-- splat:values path=splat_spell/material -->

| Field | Value |
| --- | --- |
| ` description ` | ` a gem, crystal, or reliquary worth 500+ GP ` |
| ` consumed ` | ` false ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/magic_jar ` |
| ` implementation ` | ` dnd2024:spell_magic_jar ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` projection_choices ` | ` ["return_to_living_original_body_and_end_spell", "possess_humanoid"] ` |

#### Targeting

<!-- splat:values path=splat_rule/parameters/targeting -->

| Field | Value |
| --- | --- |
| ` kind ` | ` self ` |
| ` soul_container ` | ` material_component ` |

#### Initial

<!-- splat:values path=splat_rule/parameters/initial -->

| Field | Value |
| --- | --- |
| ` body_state ` | ` catatonic ` |
| ` soul_location ` | ` container ` |

#### Container state

<!-- splat:values path=splat_rule/parameters/container_state -->

| Field | Value |
| --- | --- |
| ` perception_origin ` | ` container_space ` |
| ` movement ` | ` forbidden ` |
| ` reactions ` | ` forbidden ` |
| ` only_action ` | ` soul_projection ` |
| ` projection_max_feet ` | ` 100 ` |

#### Possession

<!-- splat:yaml path=splat_rule/parameters/possession -->

```yaml
targeting:
  types:
  - humanoid
  visible: true
  range_feet: 100
  excluded_protections:
  - protection_from_evil_and_good
  - magic_circle
save:
  ability: charisma
  failure:
    caster_soul: target_body
    target_soul: container
  success:
    possession: false
    immunity_to_caster_possession_hours: 24
host_control: caster
replace_stats:
- hit_points
- hit_point_dice
- strength
- dexterity
- constitution
- speed
- senses
retain_stats: all_other_caster_statistics
host_soul_in_container:
  perception: own_senses_from_container
  movement: forbidden
  condition: incapacitated
```

#### Leave host

<!-- splat:values path=splat_rule/parameters/leave_host -->

| Field | Value |
| --- | --- |
| ` economy ` | ` magic_action ` |
| ` container_range_max_feet ` | ` 100 ` |
| ` caster_soul ` | ` container ` |
| ` host_soul ` | ` host_body ` |

#### Host death

<!-- splat:yaml path=splat_rule/parameters/host_death -->

```yaml
host_dies: true
caster_save:
  ability: charisma
  dc: own_spell_save_dc
caster_survives_if:
- save_success
- container_within_100_feet
survival_destination: container
otherwise: caster_dies
```

#### Termination

<!-- splat:yaml path=splat_rule/parameters/termination -->

```yaml
events:
- container_destroyed
- spell_ends
caster_soul_return: original_body
caster_dies_if_any:
- original_body_dead
- original_body_more_than_100_feet_from_caster_soul
trapped_host_soul:
  return_if:
  - host_body_alive
  - host_body_within_100_feet_of_container
  otherwise: host_dies
container_destroyed_on_spell_end: true
```

#### Spell lifetime

<!-- splat:yaml path=splat_rule/parameters/spell_lifetime -->

```yaml
concentration: false
maximum:
  kind: until_dispelled
```
