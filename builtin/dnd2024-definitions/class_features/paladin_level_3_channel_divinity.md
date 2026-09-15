---
splat_schema: 1
splat_id: 459b4c2e-7c8a-5fef-9332-ee83c0b32ffd
splat_kind: rule
splat_key: dnd2024:class_features/paladin_level_3_channel_divinity
splat_name: 'Level 3: Channel Divinity'
splat_origin:
  key: dnd2024:reference/pages/054
splat_sources:
- title: SRD 5.2.1
  locator: pp. 54, 55
  url: https://www.dndbeyond.com/srd
---

# Level 3: Channel Divinity

## Source text

<!-- SRD page 54 -->

You can channel divine energy directly from the\
Outer Planes, using it to fuel magical effects. You\
start with one such effect: Divine Sense, which is\
described below. Other Paladin features give addi-\
tional Channel Divinity effect options. Each time\
you use this class’s Channel Divinity, you choose\
which effect from this class to create.\
You can use this class’s Channel Divinity twice.\
You regain one of its expended uses when you finish\
a Short Rest, and you regain all expended uses when\
you finish a Long Rest. You gain an additional use\
when you reach Paladin level 11.\
BrEakinG your oatH\
A Paladin tries to hold to the highest standards of\
conduct, but even the most dedicated are fallible.\
Sometimes a Paladin transgresses their oath.\
A Paladin who has broken a vow typically seeks\
absolution, spending an all-night vigil as a sign of pen-\
itence or undertaking a fast. After a rite of forgive-\
ness, the Paladin starts fresh.\
If your Paladin unrepentantly violates their oath,\
talk to your GM. Your Paladin should probably take a\
more appropriate subclass or even abandon the class\
and adopt another one.\

<!-- SRD page 55 -->

If a Channel Divinity effect requires a saving\
throw, the DC equals the spell save DC from this\
class’s Spellcasting feature.\
Divine Sense. As a Bonus Action, you can open\
your awareness to detect Celestials, Fiends, and\
Undead. For the next 10 minutes or until you have\
the Incapacitated condition, you know the location\
of any creature of those types within 60 feet of\
yourself, and you know its creature type. Within\
the same radius, you also detect the presence of any\
place or object that has been consecrated or dese-\
crated, as with the Hallow spell.\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/paladin_level_3_channel_divinity ` |
| ` implementation ` | ` dnd2024:class_features_paladin_level_3_channel_divinity ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` choose_effect_each_use ` | ` 1 ` |
| ` cost_each_use ` | ` 1 ` |

#### Resource

<!-- splat:values path=splat_rule/parameters/resource -->

| Field | Value |
| --- | --- |
| ` id ` | ` paladin.channel_divinity ` |

##### Maximum

<!-- splat:yaml path=splat_rule/parameters/resource/maximum -->

```yaml
by_paladin_level:
  '3': 2
  '11': 3
```

##### Recovery

<!-- splat:table
path: splat_rule/parameters/resource/recovery
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
  Details:
    type: section
    optional: true
-->

| Key | Event | Details |
| --- | --- | --- |
| ` row_1 ` | ` rest.short_completed ` | [Rules](#^splat-d8de689e-row_1 "section:row_1") |
| ` row_2 ` | ` rest.long_completed ` | [Rules](#^splat-d8de689e-row_2 "section:row_2") |

###### Row 1

^splat-d8de689e-row_1

###### Amount

<!-- splat:yaml path=splat_rule/parameters/resource/recovery/@row_1/amount -->

```yaml
1
```

###### Row 2

^splat-d8de689e-row_2

###### Amount

<!-- splat:yaml path=splat_rule/parameters/resource/recovery/@row_2/amount -->

```yaml
all
```

#### Save dc

<!-- splat:yaml path=splat_rule/parameters/save_dc -->

```yaml
op: add
args:
- 8
- var: actor.abilities.charisma.modifier
- var: actor.proficiency_bonus
```

#### Divine sense

<!-- splat:yaml path=splat_rule/parameters/divine_sense -->

```yaml
action: bonus_action
duration:
  minutes: 10
ends_on:
- actor.incapacitated
radius_feet: 60
detect_creatures:
  types:
  - celestial
  - fiend
  - undead
  reveal:
  - location
  - creature_type
detect_objects_and_places:
  tags_any:
  - consecrated
  - desecrated
  reveal:
  - presence
```

#### Class scope

<!-- splat:values path=splat_rule/parameters/class_scope -->

| Field | Value |
| --- | --- |
| ` class ` | ` paladin ` |
| ` minimum_level ` | ` 3 ` |
