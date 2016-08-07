// OpenAOE: An open source reimplementation of Age of Empires (1997)
// Copyright (c) 2016 Kevin Fuller
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use dat::EmpiresDbRef;
use ecs::component::*;
use specs::{self, Join};
use super::super::System;
use types::{Fixed, Norm, Vector3};

const THRESHOLD: Fixed = fixed_const!(0.1);

pub struct MoveToPositionActionSystem {
    empires: EmpiresDbRef,
}

impl MoveToPositionActionSystem {
    pub fn new(empires: EmpiresDbRef) -> MoveToPositionActionSystem {
        MoveToPositionActionSystem { empires: empires }
    }
}

impl System for MoveToPositionActionSystem {
    fn update(&mut self, arg: specs::RunArg, _time_step: Fixed) {
        let (mut velocities, transforms, mut units, mtps, mut action_queues) = arg.fetch(|w| {
            (w.write::<VelocityComponent>(),
             w.read::<TransformComponent>(),
             w.write::<UnitComponent>(),
             w.read::<MoveToPositionActionComponent>(),
             w.write::<ActionQueueComponent>())
        });

        let items = (&mut velocities, &transforms, &mut units, &mtps, &mut action_queues);
        for (mut velocity, transform, unit, mtps, mut action_queue) in items.iter() {
            let mut direction = mtps.target - *transform.position();
            let distance = direction.normalize();

            let done = if distance <= THRESHOLD {
                true
            } else {
                match unit.db(&self.empires).motion_params {
                    Some(ref params) => {
                        if params.walking_graphics[0].is_some() &&
                                unit.graphic_id != params.walking_graphics[0] {
                            unit.set_graphic(params.walking_graphics[0])
                        }
                        let speed: Fixed = params.speed.into();
                        velocity.velocity = direction * speed;
                        false
                    }
                    None => true,
                }
            };

            if done {
                let unit_info = unit.db(&self.empires);
                unit.set_graphic(unit_info.standing_graphic);
                velocity.velocity = Vector3::new(0.into(), 0.into(), 0.into());
                action_queue.mark_current_done();
            }
        }
    }
}