const EXPECT = { SUCCESS: "Add a defect", FAILURE: "Invalid", HTMLINPUT: "Add a defect without any html effect" };
const data = require("../fixtures/defectData2.json");

describe("Domain testing for creating a defect", () => {
   beforeEach("Precondition - Login", () => {
      cy.clearCookies();
      cy.viewport(1920, 1080);

      cy.visit("https://qa-fengine-prod.onrender.com/login");
      cy.login("hieunguyen2@gmail.com", "hieunguyen2");

      // Validation
      cy.url().should("eq", "https://qa-fengine-prod.onrender.com/dashboard");
   });

   beforeEach("Precondition - move to defect creation form", () => {
      //cy.visit("https://qa-fengine-prod.onrender.com/project/1/defect/create");
      cy.get("#project-cards-container > div > div:first-child > div").click();
      cy.url().should("include", "repository");

      cy.contains("ISSUES").click();
      cy.contains("Defects").click();
      cy.get("#select-all").click();
      cy.get("#bulk-delete").click();
      cy.on("window:confirm", () => true);
      cy.wait(500);
      cy.contains("Create new defect").click();

      cy.url().should("include", "/defect/create");
   });

   beforeEach("Add values to selection box", () => {
      // Add values for selection box
      cy.get("#defect-environment").then(($select) => {
         const newOption = `<option value="Initialization">Initialization</option>`;
         $select.append(newOption);
      });
      cy.get("#defect-severity").then(($select) => {
         const newOption = `<option value="Urgent">Urgent</option>`;
         $select.append(newOption);
      });
      cy.get("#defect-priority").then(($select) => {
         const newOption = `<option value="Moderate">Moderate</option>`;
         $select.append(newOption);
      });
      cy.get("#defect-assignee").then(($select) => {
         const newOption = `<option value="10000000">10000000</option>`;
         $select.append(newOption);
      });
   });

   data.forEach((row) =>
      it(`Test case for ${row.name}`, () => {
         cy.createDefect({ ...row });

         // Validation
         if (row.expect == EXPECT.SUCCESS) {
            cy.get(".pagination > li:nth-last-child(2)").click();

            cy.get("tbody > tr:last-child > td:nth-child(2)").should("contain", row.title);
            //cy.get("tbody > tr:last-child > td:nth-child(4)").should("contain", row.assignee);
            cy.get("tbody > tr:last-child > td:nth-child(5)").should("contain", row.severity);
         } else if (row.expect == EXPECT.FAILURE) {
            cy.contains("Error").should("be.visible");
         } else if (row.expect == EXPECT.HTMLINPUT) {
            cy.get(".pagination > li:nth-last-child(2)").click();
            cy.get("tbody > tr:last-child > td:nth-child(2) > a").invoke("text").should("include", row.title); // Have the full <h1>Hello</h1> as a innerText
         }
      })
   );
});
