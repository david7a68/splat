---
splat_schema: 1
splat_id: 44c00e76-f6b9-5f17-9c2b-03e4c383752b
splat_kind: rule
splat_key: dnd2024:rules/magical_contagions_sight_rot
splat_name: Sight Rot
splat_origin:
  key: dnd2024:reference/pages/194
splat_sources:
- title: SRD 5.2.1
  locator: pp. 194
  url: https://www.dndbeyond.com/srd
---

# Sight Rot

## Source text

<!-- SRD page 194 -->

Magical Contagion\
Any Beast or Humanoid that drinks water tainted\
by Sight Rot must succeed on a DC 15 Constitution\
saving throw or have the Blinded condition until the\
contagion ends.\
Fighting the Contagion. Magic such as a Heal or\
Lesser Restoration spell ends the contagion immedi-\
ately. A character who is proficient with an Herbal-\
ism Kit can use it to create one dose of nonmagical\
ointment, which takes 1 hour. When applied to the\
eyes of a creature suffering from Sight Rot, the oint-\
ment suppresses the contagion on that creature for\
24 hours. If the contagion is suppressed in this way\
for a total of 72 hours (requiring three doses and\
applications of the ointment), the contagion ends on\
the creature.\
Spreading the Contagion. Any Humanoid that\
makes skin contact with a creature infected with\
Sight Rot must succeed on a DC 15 Constitution\
saving throw or also become infected with the con-\
tagion. On a successful save, the Humanoid can’t\
catch the contagion from that particular infected\
creature for the next 24 hours.\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/magical_contagions_sight_rot ` |
| ` implementation ` | ` dnd2024:rule_magical_contagions_sight_rot ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` contagion ` | ` sight_rot ` |

#### Water exposure

<!-- splat:yaml path=splat_rule/parameters/water_exposure -->

```yaml
creature_types:
- beast
- humanoid
trigger: drink_tainted_water
save:
  ability: constitution
  dc: 15
  failure: infected
```

#### Infected effect

<!-- splat:values path=splat_rule/parameters/infected_effect -->

| Field | Value |
| --- | --- |
| ` condition ` | ` blinded ` |
| ` until ` | ` contagion_ends ` |

#### Immediate cures

<!-- splat:rows path=splat_rule/parameters/immediate_cures -->

| Row | key |
| --- | --- |
| ` 0 ` | ` dnd2024:spells/heal ` |
| ` 1 ` | ` dnd2024:spells/lesser_restoration ` |

#### Ointment

<!-- splat:yaml path=splat_rule/parameters/ointment -->

```yaml
craft_tool:
  key: dnd2024:equipment/other_tools_herbalism_kit_5_gp
proficiency_required: true
tool_use_required: true
hours_per_dose: 1
doses_per_craft: 1
application_location: eyes
suppression_hours_per_dose: 24
total_suppression_hours_to_cure: 72
doses_and_applications_to_cure: 3
```

#### Skin contact spread

<!-- splat:yaml path=splat_rule/parameters/skin_contact_spread -->

```yaml
recipient_type: humanoid
source: infected_creature
save:
  ability: constitution
  dc: 15
  failure: infected
  success:
    immunity_to_same_source_hours: 24
```
