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

(ns wildwestrom.death-calendar.backend.app-test
  (:require [clojure.test :refer [deftest testing is]]
            [wildwestrom.death-calendar.backend.app :as sut]
            [ring.mock.request :as mock]))

(deftest basic-request-response
  (let [resp (sut/app (mock/request :get "/"))]
     (testing "Do we get some response?"
       (is (some? resp)))
     (testing "Do we get a status 200 back?"
       (is (= 200 (:status resp))))))