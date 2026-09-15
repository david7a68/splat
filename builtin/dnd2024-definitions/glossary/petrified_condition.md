---
splat_schema: 1
splat_id: 26045ee2-4818-5c3f-b16d-4f24f274491f
splat_kind: rule
splat_key: dnd2024:glossary/petrified_condition
splat_name: Petrified [Condition]
splat_origin:
  key: dnd2024:reference/glossary/petrified_condition
splat_sources:
- title: SRD 5.2.1
  locator: pp. 186
  url: https://www.dndbeyond.com/srd
---

# Petrified [Condition]

## Source text

<!-- SRD page 186 -->

While you have the Petrified condition, you experi-\
ence the following effects.\
Turned to Inanimate Substance. You are trans-\
formed, along with any nonmagical objects you are\
wearing and carrying, into a solid inanimate sub-\
stance (usually stone). Your weight increases by a\
factor of ten, and you cease aging.\
Incapacitated. You have the Incapacitated\
condition.\
Speed 0. Your Speed is 0 and can’t increase.\
Attacks Affected. Attack rolls against you have\
Advantage.\
Saving Throws Affected. You automatically fail\
Strength and Dexterity saving throws.\
Resist Damage. You have Resistance to all\
damage.\
Poison Immunity. You have Immunity to the Poi-\
soned condition.\

Original layout: [[../../srd-5.2.1/pages/186|page 186]].

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:glossary/petrified_condition ` |
| ` implementation ` | ` dnd2024:glossary_petrified_condition ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` condition ` | ` petrified ` |
| ` grants_condition ` | ` incapacitated ` |
| ` automatic_save_failure_abilities ` | ` ["strength", "dexterity"] ` |
| ` incoming_attack_rolls ` | ` advantage ` |
| ` speed ` | ` 0 ` |
| ` speed_can_increase ` | ` false ` |
| ` damage_resistance ` | ` all ` |
| ` condition_immunity ` | ` ["poisoned"] ` |

#### Transform

<!-- splat:values path=splat_rule/parameters/transform -->

| Field | Value |
| --- | --- |
| ` targets ` | ` ["self", "worn_nonmagical_objects", "carried_nonmagical_objects"] ` |
| ` substance ` | ` solid_inanimate ` |
| ` default_substance ` | ` stone ` |
| ` weight_multiplier ` | ` 10 ` |
| ` aging_stops ` | ` true ` |
