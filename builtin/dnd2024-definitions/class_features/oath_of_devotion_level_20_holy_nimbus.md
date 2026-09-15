---
splat_schema: 1
splat_id: 071dc943-e60f-5646-9efe-315208184a24
splat_kind: rule
splat_key: dnd2024:class_features/oath_of_devotion_level_20_holy_nimbus
splat_name: 'Level 20: Holy Nimbus'
splat_origin:
  key: dnd2024:reference/pages/057
splat_sources:
- title: SRD 5.2.1
  locator: pp. 57
  url: https://www.dndbeyond.com/srd
---

# Level 20: Holy Nimbus

## Source text

<!-- SRD page 57 -->

As a Bonus Action, you can imbue your Aura of Pro-\
tection with holy power, granting the benefits be-\
low for 10 minutes or until you end them (no action\
required). Once you use this feature, you can’t use it\
again until you finish a Long Rest. You can also re-\
store your use of it by expending a level 5 spell slot\
(no action required).\
Holy Ward. You have Advantage on any saving\
throw you are forced to make by a Fiend or an\
Undead.\
Radiant Damage. Whenever an enemy starts its\
turn in the aura, that creature takes Radiant dam-\
age equal to your Charisma modifier plus your Pro-\
ficiency Bonus.\
Sunlight. The aura is filled with Bright Light that\
is sunlight.\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/oath_of_devotion_level_20_holy_nimbus ` |
| ` implementation ` | ` dnd2024:class_features_oath_of_devotion_level_20_holy_nimbus ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` action ` | ` bonus_action ` |
| ` voluntary_end_action ` | ` none ` |

#### Duration

<!-- splat:values path=splat_rule/parameters/duration -->

| Field | Value |
| --- | --- |
| ` minutes ` | ` 10 ` |

#### Uses

<!-- splat:yaml path=splat_rule/parameters/uses -->

```yaml
maximum: 1
recover:
  event: rest.long_rest_completed
  amount: all
```

#### Restore use

<!-- splat:values path=splat_rule/parameters/restore_use -->

| Field | Value |
| --- | --- |
| ` spell_slot_level ` | ` 5 ` |
| ` spell_slot_cost ` | ` 1 ` |
| ` action ` | ` none ` |

#### Benefits

##### Saving throw advantage

<!-- splat:values path=splat_rule/parameters/benefits/saving_throw_advantage -->

| Field | Value |
| --- | --- |
| ` forced_by_creature_types ` | ` ["fiend", "undead"] ` |

##### Aura damage

<!-- splat:values path=splat_rule/parameters/benefits/aura_damage -->

| Field | Value |
| --- | --- |
| ` aura ` | ` own_aura_of_protection ` |
| ` trigger ` | ` enemy.turn_started_in_aura ` |
| ` type ` | ` radiant ` |

###### Amount

<!-- splat:values path=splat_rule/parameters/benefits/aura_damage/amount -->

| Field | Value |
| --- | --- |
| ` op ` | ` add ` |

###### Args

<!-- splat:table
path: splat_rule/parameters/benefits/aura_damage/amount/args
key: Key
order:
- row_1
- row_2
columns:
  Key:
    type: key
  Stat:
    path: stat
    type: string
-->

| Key | Stat |
| --- | --- |
| ` row_1 ` | ` abilities.charisma.modifier ` |
| ` row_2 ` | ` proficiency_bonus ` |

##### Aura light

<!-- splat:values path=splat_rule/parameters/benefits/aura_light -->

| Field | Value |
| --- | --- |
| ` brightness ` | ` bright ` |
| ` sunlight ` | ` true ` |
