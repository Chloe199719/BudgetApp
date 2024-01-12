-- Add up migration script here
ALTER TABLE categories ADD COLUMN budget_Id int;
ALTER TABLE categories ADD FOREIGN KEY (budget_Id) REFERENCES budgets(budget_id);
