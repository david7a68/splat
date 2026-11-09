---
splat_schema: 2
splat_id: 62767181-90ee-5077-b4b3-73e185eb0d10
splat_kind: spell
splat_key: dnd2024:spells/imprisonment
splat_name: Imprisonment
splat_origin: dnd2024:reference/spell/imprisonment
splat_sources:
- SRD 5.2.1, pp. 142, https://www.dndbeyond.com/srd
---

# Imprisonment


## Spell

| Field | Value |
| --- | --- |
| Level | 9 |
| School | Abjuration |
| Classes | Warlock, Wizard |
| Casting Time | 1 minute |
| Range | 30 feet |
| Components | V, S, M (a statuette of the target worth 5,000+ GP) |
| Duration | Until dispelled |
| Ritual | No |
| Concentration | No |


## Source text

<!-- SRD page 142 -->

Level 9 Abjuration (Warlock, Wizard)\
Casting Time: 1 minute\
Range: 30 feet\
Components: V, S, M (a statuette of the target worth 5,000+ GP)\
Duration: Until dispelled

You create a magical restraint to hold a creature that you can see within range. The target must make a Wisdom saving throw. On a successful save, the target is unaffected, and it is immune to this spell for the next 24 hours. On a failed save, the target is imprisoned. While imprisoned, the target doesn’t need to breathe, eat, or drink, and it doesn’t age. Divination spells can’t locate or perceive the imprisoned target, and the target can’t teleport.

Until the spell ends, the target is also affected by one of the following effects of your choice:

Burial. The target is entombed beneath the earth in a hollow globe of magical force that is just large enough to contain the target. Nothing can pass into or out of the globe.

Chaining. Chains firmly rooted in the ground hold the target in place. The target has the Restrained condition and can’t be moved by any means.

Hedged Prison. The target is trapped in a demiplane that is warded against teleportation and planar travel. The demiplane is your choice of a labyrinth, a cage, a tower, or the like.

Minimus Containment. The target becomes 1 inch tall and is trapped inside an indestructible gemstone or a similar object. Light can pass through the gemstone (allowing the target to see out and other creatures to see in), but nothing else can pass through by any means.

Slumber. The target has the Unconscious condition and can’t be awoken.

Ending the Spell. When you cast the spell, specify a trigger that will end it. The trigger can be as simple or as elaborate as you choose, but the GM must agree that it has a high likelihood of happening within the next decade. The trigger must be an observable action, such as someone making a particular offering at the temple of your god, saving your true love, or defeating a specific monster. A Dispel Magic spell can end the spell only if it is cast with a level 9 spell slot, targeting either the prison or the component used to create it.

Original layout: [[../../srd-5.2.1/pages/142|page 142]].







## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/imprisonment ` |
| ` implementation ` | ` dnd2024:spell_imprisonment ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` mode_choice ` | ` ["burial", "chaining", "hedged_prison", "minimus_containment", "slumber"] ` |

#### Target

<!-- splat:values path=splat_rule/parameters/target -->

| Field | Value |
| --- | --- |
| ` kind ` | ` creature ` |
| ` count ` | ` 1 ` |
| ` visible_to_caster ` | ` true ` |

#### Save

<!-- splat:yaml path=splat_rule/parameters/save -->

```yaml
ability: wisdom
dc:
  binding: save_dc
failure:
  imprisoned: true
success:
  unaffected: true
  immunity_to_spell_hours: 24
```

#### Imprisoned

<!-- splat:values path=splat_rule/parameters/imprisoned -->

| Field | Value |
| --- | --- |
| ` breathing_required ` | ` false ` |
| ` food_required ` | ` false ` |
| ` drink_required ` | ` false ` |
| ` aging ` | ` false ` |
| ` divination_locate_or_perceive ` | ` false ` |
| ` teleport ` | ` false ` |

#### Modes

<!-- splat:yaml path=splat_rule/parameters/modes -->

```yaml
burial:
  location: beneath_earth
  container: hollow_force_globe
  size: just_fits_target
  pass_in_or_out: false
chaining:
  condition: restrained
  anchored: ground
  move_by_any_means: false
hedged_prison:
  location: demiplane
  teleportation: false
  planar_travel: false
  appearance_choice: prison_form
minimus_containment:
  height_inches: 1
  container: gemstone_or_similar_object
  container_indestructible: true
  light_passes: true
  anything_else_passes: false
slumber:
  condition: unconscious
  can_awaken: false
```

#### Ending trigger

<!-- splat:yaml path=splat_rule/parameters/ending_trigger -->

```yaml
chooser: caster
decision_type: observable_action_predicate
gm_approval_required: true
gm_constraint:
  high_likelihood_within_years: 10
chosen_at: cast
```

#### Dispel magic

<!-- splat:values path=splat_rule/parameters/dispel_magic -->

| Field | Value |
| --- | --- |
| ` required_slot_level ` | ` 9 ` |
| ` eligible_targets ` | ` ["prison", "casting_component"] ` |

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_spell_extras -->
> 
> ```yaml
> material:
>   consumed: false
> ```

