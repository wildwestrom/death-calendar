;; Death Calendar: See how many days you have left to live at a glance.
;; Copyright © 2021 Christian Westrom

;; This program is free software: you can redistribute it and/or modify
;; it under the terms of the GNU Affero General Public License as
;; published by the Free Software Foundation, either version 3 of the
;; License, or (at your option) any later version.

;; This program is distributed in the hope that it will be useful,
;; but WITHOUT ANY WARRANTY; without even the implied warranty of
;; MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
;; GNU Affero General Public License for more details.

;; You should have received a copy of the GNU Affero General Public License
;; along with this program.  If not, see <https://www.gnu.org/licenses/>.

(ns wildwestrom.death-calendar.core
  (:require [clojure.spec.alpha :as s]
            [clojure.spec.gen.alpha :as gen]
            [tick.core :as t]))

(defn death-day
  [birth-day life-span]
  (t/>> birth-day life-span))

(defn total-lifespan
  [birth-day life-span]
  (let [birth-day (when (t/date? birth-day)
                    (-> birth-day (t/at "00:00")))]
    (t/duration {:tick/beginning (t/instant birth-day)
                 :tick/end (t/instant (death-day birth-day life-span))})))

(defn life-lived
  [birth-day]
  (let [birth-day (when (t/date? birth-day)
                    (-> birth-day (t/at "00:00")))]
    (t/duration {:tick/beginning (t/instant birth-day)
                 :tick/end  (t/instant)})))

(defn life-remaining
  [birth-day life-span]
  (let [birth-day (when (t/date? birth-day)
                    (-> birth-day (t/at "00:00")))]
    (t/duration {:tick/beginning (t/instant)
                 :tick/end  (t/instant (death-day birth-day life-span))})))

(defn is-alive [birth-day life-span]
  (let [remaining (life-remaining birth-day life-span)]
    (if (> 0 (t/days remaining))
      false
      true)))

(defn calendar-data
  [birth-day life-span]
  (let [unit-fn t/days
        total-life (total-lifespan birth-day life-span)
        remaining  (life-remaining birth-day life-span)
        alive      (is-alive birth-day life-span)
        lived      (life-lived birth-day)]
    {:unit      unit-fn
     :total     total-life
     :lived     lived
     :remaining remaining
     :alive     alive}))

(def estimated-life (t/new-period 80 :years))

#_(-> (t/new-date 2000 2 18)
      (calendar-data estimated-life)
      :remaining
      t/days
      int)
