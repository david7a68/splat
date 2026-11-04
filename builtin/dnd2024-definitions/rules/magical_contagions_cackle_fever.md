---
splat_schema: 2
splat_id: 1afbbc28-9002-5d09-b54b-978427fb3c01
splat_kind: rule
splat_key: dnd2024:rules/magical_contagions_cackle_fever
splat_name: Cackle Fever
splat_origin: dnd2024:reference/pages/194
splat_sources:
- SRD 5.2.1, pp. 194, https://www.dndbeyond.com/srd
---

# Cackle Fever

## Source text

<!-- SRD page 194 -->

Magical Contagion

Cheaply made potions and elixirs are sometimes tainted by Cackle Fever, which affects Humanoids only (gnomes are strangely immune). A creature suffers the following effects 1d4 days after infection:

Fever. The creature gains 1 Exhaustion level, which lasts until the contagion ends on the creature.

Uncontrollable Laughter. While the creature has the Exhaustion condition, the creature makes a DC 13 Constitution saving throw each time it takes damage other than Psychic damage. On a failed save, the creature takes 5 (1d10) Psychic damage and has the Incapacitated condition as it laughs uncontrollably. At the end of each of its turns, the creature repeats the save, ending the effect on itself on a success. After 1 minute, it succeeds automatically.

Fighting the Contagion. At the end of each Long Rest, an infected creature makes a DC 13 Constitution saving throw. After the creature succeeds on three of these saves, the contagion ends on it, and the creature is immune to Cackle Fever for 1 year.

Spreading the Contagion. Any Humanoid (other than a gnome) that starts its turn within a 10-foot Emanation originating from a creature infected with Cackle Fever must succeed on a DC 10 Constitution saving throw or also become infected with the contagion. On a successful save, the Humanoid can’t catch the contagion from that particular infected creature for the next 24 hours.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/magical_contagions_cackle_fever ` |
| ` implementation ` | ` dnd2024:rules_magical_contagions_cackle_fever ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` contagion ` | ` cackle_fever ` |
| ` eligible_type ` | ` humanoid ` |
| ` immune_species ` | ` ["gnome"] ` |

#### Onset delay

<!-- splat:yaml path=splat_rule/parameters/onset_delay -->

```yaml
dice:
  count: 1
  sides: 4
  add: 0
unit: days
```

#### Onset exhaustion

<!-- splat:values path=splat_rule/parameters/onset_exhaustion -->

| Field | Value |
| --- | --- |
| ` levels ` | ` 1 ` |
| ` duration ` | ` until_contagion_ends ` |

#### Laughter

<!-- splat:yaml path=splat_rule/parameters/laughter -->

```yaml
requires_exhaustion: true
trigger: damage_taken
excluded_damage_types:
- psychic
save:
  ability: constitution
  dc: 13
failure:
  damage:
    dice:
      count: 1
      sides: 10
      add: 0
    average: 5
    type: psychic
  condition: incapacitated
repeat_save:
  event: affected_turn.ended
  success: end_laughter
automatic_success_after:
  value: 1
  unit: minute
```

#### Recovery

<!-- splat:yaml path=splat_rule/parameters/recovery -->

```yaml
event: long_rest.finished
save:
  ability: constitution
  dc: 13
success_count: 3
consecutive_required: false
success_threshold: end_contagion
subsequent_immunity:
  value: 1
  unit: year
```

#### Spread

<!-- splat:yaml path=splat_rule/parameters/spread -->

```yaml
event: susceptible_turn.started
area:
  shape: emanation
  radius_feet: 10
  origin: infected_creature
save:
  ability: constitution
  dc: 10
failure: infect
success_immunity:
  scope: this_infected_source
  duration:
    value: 24
    unit: hours
```
