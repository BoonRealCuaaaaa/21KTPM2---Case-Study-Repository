const EXPECT = {
   SUCCESS: "Add a test case",
   FAILURE: "Invalid input",
   HTMLINPUT: "Add a test case, without any effect of html",
};

const data = require("../fixtures/testcaseData2.json");

describe("Domain testing for creating a test case", () => {
   beforeEach("Precondition - Login", () => {
      cy.clearCookies();
      cy.viewport(1920, 1080);

      cy.visit("https://qa-fengine-prod.onrender.com/login");
      cy.login("hieunguyen2@gmail.com", "hieunguyen2");

      // Validation
      cy.url().should("eq", "https://qa-fengine-prod.onrender.com/dashboard");
   });

   beforeEach("Precondition - move to test case creation form", () => {
      //cy.visit("https://qa-fengine-prod.onrender.com/project/8/test-case/create-test-case");

      cy.get("#project-cards-container > div > div:first-child > div").click();
      cy.url().should("include","repository")

      cy.contains("+ Test Case").click()

      cy.url().should("include", "create-test-case");
   });

   beforeEach("Add values to selection box", () => {
      // Status
      cy.get("#test-case-status").then(($select) => {
         const newOption = `<option value="Very Low">Very Low</option>`;
         $select.append(newOption);
      });

      // Priority
      cy.get("#test-case-priority").then(($select) => {
         const newOption = `<option value="Ultra Low">Ultra Low</option>`;
         $select.append(newOption);
      });

      // Severity
      cy.get("#test-case-severity").then(($select) => {
         const newOption = `<option value="Low">Low</option>`;
         $select.append(newOption);
      });

      // Suite
      cy.get("#test-case-suite").then(($select) => {
         const newOption = `<option value="Test case in security suite">Test case in security suite</option>`;
         $select.append(newOption);
      });
   });

   data.forEach((row) =>
      it(`Test case for ${row.name}`, () => {
         cy.createTestCase({ ...row });

         // Validation
         if (row.expect == EXPECT.SUCCESS) {
            cy.get("#testCasesContainer > div:last-child > div > button:last-child")
               .invoke("text")
               .should("include", row.title);
         } else if (row.expect == EXPECT.FAILURE) {
            cy.contains("Error").should("be.visible");
         } else if (row.expect == EXPECT.HTMLINPUT) {
            cy.get("#testCasesContainer > div:last-child > div > button:last-child")
               .invoke("text")
               .should("include", row.title);
         }
      })
   );
});
