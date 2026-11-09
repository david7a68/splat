---
splat_schema: 2
splat_id: aaa54732-7c39-5852-949a-6d51bbaad7ed
splat_kind: spell
splat_key: dnd2024:spells/flesh_to_stone
splat_name: Flesh to Stone
splat_origin: dnd2024:reference/spell/flesh_to_stone
splat_sources:
- SRD 5.2.1, pp. 133, https://www.dndbeyond.com/srd
---

# Flesh to Stone


## Spell

| Field | Value |
| --- | --- |
| Level | 6 |
| School | Transmutation |
| Classes | Druid, Sorcerer, Wizard |
| Casting Time | Action |
| Range | 60 feet |
| Components | V, S, M (a cockatrice feather) |
| Duration | Concentration, up to 1 minute |
| Ritual | No |
| Concentration | Yes |


## Source text

<!-- SRD page 133 -->

Level 6 Transmutation (Druid, Sorcerer, Wizard)\
Casting Time: Action\
Range: 60 feet\
Components: V, S, M (a cockatrice feather)\
Duration: Concentration, up to 1 minute

You attempt to turn one creature that you can see within range into stone. The target makes a Constitution saving throw. On a failed save, it has the Restrained condition for the duration. On a successful save, its Speed is 0 until the start of your next turn. Constructs automatically succeed on the save.

A Restrained target makes another Constitution saving throw at the end of each of its turns. If it successfully saves against this spell three times, the spell ends. If it fails its saves three times, it is turned to stone and has the Petrified condition for the duration. The successes and failures needn’t be consecutive; keep track of both until the target collects three of a kind.

If you maintain your Concentration on this spell for the entire possible duration, the target is Petrified until the condition is ended by Greater Restoration or similar magic.

Original layout: [[../../srd-5.2.1/pages/133|page 133]].







## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/flesh_to_stone ` |
| ` implementation ` | ` dnd2024:spell_flesh_to_stone ` |

### Parameters

#### Targeting

<!-- splat:values path=splat_rule/parameters/targeting -->

| Field | Value |
| --- | --- |
| ` kind ` | ` creature ` |
| ` count ` | ` 1 ` |
| ` visible ` | ` true ` |
| ` within_range ` | ` true ` |

#### Initial save

<!-- splat:yaml path=splat_rule/parameters/initial_save -->

```yaml
ability: constitution
automatic_success_types:
- construct
failure:
  condition: restrained
  duration: spell_duration
success:
  speed: 0
  until: caster_next_turn_start
```

#### Progression

<!-- splat:yaml path=splat_rule/parameters/progression -->

```yaml
event: restrained_target.turn_ended
save_ability: constitution
counters:
  scope: saves_against_this_spell
  initial_failure_counts:
    type: choice
    chooser: table
    options:
    - true
    - false
    source_scope: saves_against_this_spell
  consecutive_required: false
  success_threshold: 3
  failure_threshold: 3
on_success_threshold: end_spell
on_failure_threshold:
  form: stone
  condition: petrified
  duration: spell_duration
```

#### Full concentration duration

<!-- splat:values path=splat_rule/parameters/full_concentration_duration -->

| Field | Value |
| --- | --- |
| ` condition ` | ` petrified ` |
| ` duration ` | ` until_removed ` |
| ` removal ` | ` ["greater_restoration", "similar_magic"] ` |

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
> material:
>   consumed: false
> ```

