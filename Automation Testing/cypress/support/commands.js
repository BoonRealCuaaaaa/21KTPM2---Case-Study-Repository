// ***********************************************
// This example commands.js shows you how to
// create various custom commands and overwrite
// existing commands.
//
// For more comprehensive examples of custom
// commands please read more here:
// https://on.cypress.io/custom-commands
// ***********************************************
//
//
// -- This is a parent command --
// Cypress.Commands.add('login', (email, password) => { ... })
//
//
// -- This is a child command --
// Cypress.Commands.add('drag', { prevSubject: 'element'}, (subject, options) => { ... })
//
//
// -- This is a dual command --
// Cypress.Commands.add('dismiss', { prevSubject: 'optional'}, (subject, options) => { ... })
//
//
// -- This will overwrite an existing command --
// Cypress.Commands.overwrite('visit', (originalFn, url, options) => { ... })
Cypress.Commands.add("login", (username, password) => {
   cy.get("#email").type(username);
   cy.get("#password").type(password);

   cy.get("button[type='submit']").click();
});

Cypress.Commands.add("createDefect", ({ title, desc, step, env, severity, priority, assignee }) => {
   if (title) cy.get("#defect-title").type(title);
   if (desc) cy.get("#defect-description").type(desc);
   if (step) cy.get("#defect-step").type(step);

   cy.get("#defect-environment").select(env);
   cy.get("#defect-severity").select(severity);
   cy.get("#defect-priority").select(priority);
   cy.get("#defect-assignee").select(assignee);

   // Submit form
   cy.get("#save-defect").click();
});

Cypress.Commands.add(
   "createTestCase",
   ({ title, desc, status, priority, severity, suite, pre, post, step, expectedResult }) => {
      cy.get("#add-step-button").click();

      if (title) cy.get("#test-case-name").type(title);
      if (desc) cy.get("#test-case-description").type(desc);
      if (pre) cy.get("#pre-conditions").type(pre);
      if (post) cy.get("#post-conditions").type(post);
      if (step) cy.get("#step-action-0").type(step);
      if (expectedResult) cy.get("#expected-result-0").type(expectedResult);

      cy.get("#test-case-status").select(status);
      cy.get("#test-case-priority").select(priority);
      cy.get("#test-case-severity").select(severity);
      cy.get("#test-case-suite").select(suite);

      // Submit form
      cy.get("button[type='submit']").click();
   }
);

Cypress.Commands.add("createTestRun", ({ testRunName, desc, env, tags, type, tc }) => {
   cy.get(".btn.btn-primary.mt-4").click();
   cy.wait(500); // Race condition prevent

   if (testRunName) cy.get("#test-run-name").type(testRunName);
   if (desc) cy.get("#test-run-description").type(desc);

   cy.get("#environmentSelect").select(env);
   cy.get("#tagSelect").select(tags);
   cy.get("#runTypeSelect").select(type);

   cy.contains("+ Add cases").click();
   cy.contains("Test case without suite").click();
   // Test case adding
   cy.get("#test-cases-container").then(($select) => {
      const newOption = `<option value="100000000">100000000</option>`;
      $select.append(newOption);
   });
   cy.contains(tc).click().wait(500); // Race condition prevent
   cy.get("#add-test-case-btn").click();

   // Submit form
   cy.get("#createNewRun").click();
});
