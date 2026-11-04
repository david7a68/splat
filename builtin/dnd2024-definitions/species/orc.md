---
splat_schema: 2
splat_id: 354ce3e5-2276-5504-a741-ec46d2f38de5
splat_kind: rule_group
splat_key: dnd2024:species/orc
splat_name: Orc
splat_origin: dnd2024:reference/pages/086
splat_sources:
- SRD 5.2.1, pp. 86, https://www.dndbeyond.com/srd
---

# Orc

## Source text

<!-- SRD page 86 -->

Creature Type: Humanoid\
Size: Medium (about 6–7 feet tall)\
Speed: 30 feet

As an Orc, you have these special traits.

Adrenaline Rush. You can take the Dash action as a Bonus Action. When you do so, you gain a number of Temporary Hit Points equal to your Proficiency Bonus.

You can use this trait a number of times equal to your Proficiency Bonus, and you regain all expended uses when you finish a Short or Long Rest.

Darkvision. You have Darkvision with a range of 120 feet.

Relentless Endurance. When you are reduced to 0 Hit Points but not killed outright, you can drop to 1 Hit Point instead. Once you use this trait, you can’t do so again until you finish a Long Rest.

## Rule group

<!-- splat:values path=splat_rule_group -->

| Field | Value |
| --- | --- |
| ` category ` | ` species ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:species/orc ` |
| ` implementation ` | ` dnd2024:species_orc ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` creature_type ` | ` humanoid ` |
| ` size ` | ` medium ` |
| ` speed_feet ` | ` 30 ` |
| ` darkvision_feet ` | ` 120 ` |

#### Height feet

<!-- splat:values path=splat_rule/parameters/height_feet -->

| Field | Value |
| --- | --- |
| ` typical_minimum ` | ` 6 ` |
| ` typical_maximum ` | ` 7 ` |

#### Adrenaline rush

<!-- splat:values path=splat_rule/parameters/adrenaline_rush -->

| Field | Value |
| --- | --- |
| ` action ` | ` bonus_action ` |
| ` take_action ` | ` dash ` |
| ` cost ` | ` 1 ` |

##### Temporary hit points

<!-- splat:values path=splat_rule/parameters/adrenaline_rush/temporary_hit_points -->

| Field | Value |
| --- | --- |
| ` var ` | ` actor.proficiency_bonus ` |

##### Resource

<!-- splat:values path=splat_rule/parameters/adrenaline_rush/resource -->

| Field | Value |
| --- | --- |
| ` id ` | ` orc.adrenaline_rush ` |

###### Maximum

<!-- splat:values path=splat_rule/parameters/adrenaline_rush/resource/maximum -->

| Field | Value |
| --- | --- |
| ` var ` | ` actor.proficiency_bonus ` |

###### Recovery

<!-- splat:table
path: splat_rule/parameters/adrenaline_rush/resource/recovery
key: Key
order:
- row_1
- row_2
columns:
  Key:
    type: key
  Event:
    path: event
    type: string
  Amount:
    path: amount
    type: symbol
-->

| Key | Event | Amount |
| --- | --- | --- |
| ` row_1 ` | ` rest.short_completed ` | ` All ` |
| ` row_2 ` | ` rest.long_completed ` | ` All ` |

#### Relentless endurance

<!-- splat:yaml path=splat_rule/parameters/relentless_endurance -->

```yaml
trigger: actor.reduced_to_zero_hit_points
requires:
  killed_outright: false
optional: true
instead_hit_points: 1
resource:
  id: orc.relentless_endurance
  maximum: 1
  recovery:
  - event: rest.long_completed
    amount: all
cost: 1
```
