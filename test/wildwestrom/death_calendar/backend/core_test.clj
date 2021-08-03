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
(ns wildwestrom.death-calendar.backend.core-test
  (:require [clojure.test :refer [deftest is testing use-fixtures]]
            [ring.mock.request :refer [request]]
            [wildwestrom.death-calendar.backend.core :as sut]))

(deftest home-page
  (let [resp (sut/app (request :get "/"))]
    (testing "I get a status of 200 when I do a get request on the root."
      (is (= 200 (:status resp))))
    (testing "My body has a greeting."
      (is (= sut/home-body (:body resp))))
    (testing "The content-type is HTML."
      (is (= {"html" "text/html"} (:content-type resp))))
    (testing "It's UTF8."
      (is (= "UTF8" (:charset resp))))))
