const EXPECT = {
   SUCCESS: "Create a test run",
   FAILURE: "Raise error",
   HTMLINPUT: "Create a test run, without HTML effect",
};

const data = require("../fixtures/testrunData2.json");

describe("Domain testing for creating a test run", () => {
   beforeEach("Precondition - Login", () => {
      cy.clearCookies();
      cy.viewport(1920, 1080);

      cy.visit("https://qa-fengine-prod.onrender.com/login");
      cy.login("hieunguyen2@gmail.com", "hieunguyen2");

      // Validation
      cy.url().should("eq", "https://qa-fengine-prod.onrender.com/dashboard");
   });

   beforeEach("Precondition - move to project test run", () => {
      // cy.visit("https://qa-fengine-prod.onrender.com/project/1/testrun");
      cy.get("#project-cards-container > div > div:first-child > div").click();
      cy.url().should("include","repository")

      cy.contains("EXECUTION").click();
      cy.contains("Test runs").click();

      cy.url().should("include", "/testrun");
   });

   beforeEach("Add values to selection box", () => {
      // Environment
      cy.get("#environmentSelect").then(($select) => {
         const newOption = `<option value="env03">env03</option>`;
         $select.append(newOption);
      });

      // Tags
      cy.get("#tagSelect").then(($select) => {
         const newOption = `<option value="hybrid">Hybrid</option>`;
         $select.append(newOption);
      });

      // Type
      cy.get("#runTypeSelect").then(($select) => {
         const newOption = `<option value="hybrid">Hybrid</option>`;
         $select.append(newOption);
      });


   });

   data.forEach((row) =>
      it(`Test case for ${row.name}`, () => {
         cy.createTestRun({ ...row });

         // Validation
         if (row.expect == EXPECT.SUCCESS) {
            cy.get(".table tbody > tr:last-child > td:nth-child(1)").should("contain", row.testRunName);
         } else if (row.expect == EXPECT.FAILURE) {
            cy.contains("Error").should("be.visible");
         } else if (row.expect == EXPECT.HTMLINPUT) {
            cy.get(".table tbody > tr:last-child > td:nth-child(1)").invoke("text").should("include", row.testRunName); // Have the full <h1>Hello</h1> as a string
         }
      })
   );
});
