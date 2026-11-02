---
splat_schema: 1
splat_id: 18db71b0-4ee1-5f36-a04b-c06c1090ba46
splat_kind: rule_group
splat_key: dnd2024:species/gnome
splat_name: Gnome
splat_origin:
  key: dnd2024:reference/pages/085
splat_sources:
- title: SRD 5.2.1
  locator: pp. 85
  url: https://www.dndbeyond.com/srd
---

# Gnome

## Source text

<!-- SRD page 85 -->

Creature Type: Humanoid\
Size: Small (about 3–4 feet tall)\
Speed: 30 feet

As a Gnome, you have these special traits.

Darkvision. You have Darkvision with a range of 60 feet.

Gnomish Cunning. You have Advantage on Intelligence, Wisdom, and Charisma saving throws.

Gnomish Lineage. You are part of a lineage that grants you supernatural abilities. Choose one of the following options; whichever one you choose, Intelligence, Wisdom, or Charisma is your spellcasting ability for the spells you cast with this trait (choose the ability when you select the lineage):

Forest Gnome. You know the Minor Illusion cantrip. You also always have the Speak with Animals spell prepared. You can cast it without a spell slot a number of times equal to your Proficiency Bonus, and you regain all expended uses when you finish a Long Rest. You can also use any spell slots you have to cast the spell.

Rock Gnome. You know the Mending and Prestidigitation cantrips. In addition, you can spend 10 minutes casting Prestidigitation to create a Tiny clockwork device (AC 5, 1 HP), such as a toy, fire starter, or music box. When you create the device, you determine its function by choosing one effect from Prestidigitation; the device produces that effect whenever you or another creature takes a Bonus Action to activate it with a touch. If the chosen effect has options within it, you choose one of those options for the device when you create it. For example, if you choose the spell’s ignite-extinguish effect, you determine whether the device ignites or extinguishes fire; the device doesn’t do both. You can have three such devices in existence at a time, and each falls apart 8 hours after its creation or when you dismantle it with a touch as a Utilize action.

## Rule group

<!-- splat:values path=splat_rule_group -->

| Field | Value |
| --- | --- |
| ` category ` | ` species ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:species/gnome ` |
| ` implementation ` | ` dnd2024:species_gnome ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` creature_type ` | ` humanoid ` |
| ` size ` | ` small ` |
| ` speed_feet ` | ` 30 ` |
| ` darkvision_feet ` | ` 60 ` |
| ` save_advantage_abilities ` | ` ["intelligence", "wisdom", "charisma"] ` |

#### Height feet

<!-- splat:values path=splat_rule/parameters/height_feet -->

| Field | Value |
| --- | --- |
| ` typical_minimum ` | ` 3 ` |
| ` typical_maximum ` | ` 4 ` |

#### Lineage choice

<!-- splat:yaml path=splat_rule/parameters/lineage_choice -->

```yaml
count: 1
spellcasting_ability_choice:
- intelligence
- wisdom
- charisma
ability_choice_time: lineage_selection
options:
  forest:
    cantrips:
    - key: dnd2024:spells/minor_illusion
    always_prepared:
    - key: dnd2024:spells/speak_with_animals
    free_cast:
      spell:
        key: dnd2024:spells/speak_with_animals
      resource:
        id: gnome.speak_with_animals
        maximum:
          var: actor.proficiency_bonus
        recovery:
        - event: rest.long_completed
          amount: all
      cost: 1
      expend_slot: false
    cast_with_available_slots: true
  rock:
    cantrips:
    - key: dnd2024:spells/mending
    - key: dnd2024:spells/prestidigitation
    clockwork_device:
      creation:
        cast:
          key: dnd2024:spells/prestidigitation
        minutes: 10
      size: tiny
      armor_class: 5
      hit_points: 1
      effect_choice:
        source_spell:
          key: dnd2024:spells/prestidigitation
        count: 1
        nested_options_chosen_at_creation: true
        fixed_until_destroyed: true
      activation:
        action: bonus_action
        requires_touch: true
        user: any_creature
        produce_chosen_effect: true
      maximum_simultaneous_devices: 3
      expires:
        hours_after_creation: 8
      dismantle:
        actor: creator
        action: utilize
        requires_touch: true
```
