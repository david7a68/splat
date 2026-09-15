---
splat_schema: 1
splat_id: 66bb1cfd-268b-551e-be75-e684ffba10b9
splat_kind: rule
splat_key: dnd2024:rules/traps_parts_of_a_trap
splat_name: Parts of a Trap
splat_origin:
  key: dnd2024:reference/pages/199
splat_sources:
- title: SRD 5.2.1
  locator: pp. 199
  url: https://www.dndbeyond.com/srd
---

# Parts of a Trap

## Source text

<!-- SRD page 199 -->

The description of a trap includes the following\
parts after the trap’s name:\
Severity and Levels. A trap is designated as a nui-\
sance or as deadly for characters of certain levels.\
A nuisance trap is unlikely to seriously harm char-\
acters of the indicated levels, whereas a deadly\
trap can grievously damage characters of the indi-\
cated levels.\
Trigger. Traps are often set to go off when a crea-\
ture enters an area or touches an object. Examples\
of triggers include stepping on a pressure plate,\
crossing a trip wire, turning a doorknob, or using\
the wrong key in a lock.\
Duration. Some traps have durations expressed\
in rounds, minutes, or hours. Others specify that\
their effects last until the trap is destroyed or\
dispelled. If a trap’s duration is instantaneous, its\
effect is resolved instantly. If a trap resets after\
activating, that fact is noted in this entry. A trap is\
otherwise inert after activation.\
Use caution when introducing a trap to characters\
of a level lower than the trap’s level range. A trap\
that is a nuisance at one level range could be deadly\
to characters of a lower level range.\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/traps_parts_of_a_trap ` |
| ` implementation ` | ` dnd2024:rules_traps_parts_of_a_trap ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` severity_options ` | ` ["nuisance", "deadly"] ` |
| ` duration_forms ` | ` ["rounds", "minutes", "hours", "until_destroyed", "until_dispelled", "instantaneous"] ` |
| ` after_activation_default ` | ` inert ` |
| ` reset_requires_explicit_rule ` | ` true ` |
