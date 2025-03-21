Feature: Authenication feature

  Scenario: If we log in with correct credentials we will login
    Given a webdriver
    When we go to /login
    When we input correct credentials
    Then we are logged in
