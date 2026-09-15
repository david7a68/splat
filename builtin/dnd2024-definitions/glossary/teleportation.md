---
splat_schema: 1
splat_id: 7cf6c66f-fd69-576d-8ece-f0f2a6c37d83
splat_kind: rule
splat_key: dnd2024:glossary/teleportation
splat_name: Teleportation
splat_origin:
  key: dnd2024:reference/glossary/teleportation
splat_sources:
- title: SRD 5.2.1
  locator: pp. 190
  url: https://www.dndbeyond.com/srd
---

# Teleportation

## Source text

<!-- SRD page 190 -->

Teleportation is a special kind of magical transpor-\
tation. If you teleport, you disappear and reappear\
elsewhere instantly, without moving through the\
intervening space. This transportation doesn’t ex-\
pend movement unless a rule tells you otherwise,\
and teleportation never provokes Opportunity\
Attacks.\
When you teleport, all the equipment you’re\
wearing and carrying teleports with you. If you’re\
touching another creature when you teleport, that\
creature doesn’t teleport with you unless the tele-\
portation effect says otherwise.\
If the destination space of your teleportation is\
occupied by another creature or blocked by a solid\
obstacle, you instead appear in the nearest unoccu-\
pied space of your choice.\
The description of a teleportation effect tells you\
if you must see the teleportation’s destination.\

Original layout: [[../../srd-5.2.1/pages/190|page 190]].

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:glossary/teleportation ` |
| ` implementation ` | ` dnd2024:glossary_teleportation ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` magical ` | ` true ` |
| ` instantaneous ` | ` true ` |
| ` traverses_intervening_space ` | ` false ` |
| ` movement_cost_default ` | ` 0 ` |
| ` explicit_movement_cost_override_allowed ` | ` true ` |
| ` provokes_opportunity_attacks ` | ` false ` |
| ` accompanying_equipment ` | ` all_worn_and_carried ` |
| ` touching_creatures_accompany_by_default ` | ` false ` |
| ` effect_may_allow_accompanying_creatures ` | ` true ` |
| ` destination_sight_required ` | ` source_effect_defined ` |
| ` geometry_provider ` | ` external_geometry ` |

#### Blocked destination

<!-- splat:values path=splat_rule/parameters/blocked_destination -->

| Field | Value |
| --- | --- |
| ` blocked_if_any ` | ` ["another_creature_present", "solid_obstacle"] ` |
| ` replacement ` | ` nearest_unoccupied_space ` |
| ` equal_nearest_choice ` | ` teleporting_creature ` |
