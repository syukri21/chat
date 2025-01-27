// @ts-check
import {test, expect} from '@playwright/test';
import {BASE_URL, EMAIL_2, PASSWORD, USERNAME, USERNAME_2} from "../commons/constant";
import assert from "node:assert";

test.describe('Login Flow', () => {

    test.beforeEach(async ({page}) => {
        await page.goto(`${BASE_URL}/login`);
        await page.waitForTimeout(1000);
    });

    test('should show error for non-existent user', async ({page}) => {
        await page.getByLabel('Username').fill('nonexistentuser');
        await page.getByLabel('Password').fill('password123');
        await page.getByRole('button', {name: 'Sign In'}).click();

        await expect(page.getByText('Login failed, No user and password found')).toBeVisible();
    });

    test.describe.serial("Login Flow after Success Registration", () => {
        const username = USERNAME_2
        const email = EMAIL_2
        const password = PASSWORD

        test('should show error for incorrect password', async ({page}) => {

            // First register a user
            await page.goto(`${BASE_URL}/signup`);
            await page.waitForTimeout(1000)
            await page.getByLabel('Username').fill(username);
            await page.getByLabel('Email').fill(email);
            await page.getByLabel('Password', {exact: true}).fill(password);
            await page.getByLabel('Confirm Password', {exact: true}).fill(password);
            await page.getByRole('button', {name: 'Register'}).click();

            await page.goto(`${BASE_URL}/login`);
            await page.waitForTimeout(1000)
            await page.getByLabel('Username').fill(username);
            await page.getByLabel('Password').fill('wrongpassword');
            await page.getByRole('button', {name: 'Sign In'}).click();

            await expect(page.getByText('Login failed, No user and password found')).toBeVisible();
        });

        test('should redirect to home when success login', async ({page}) => {
            // Get and parse the activation link
            const response = await page.goto(`${BASE_URL}/debug/active-link`);
            const responseBody = JSON.parse(await response.json());
            const activationLink = responseBody[username];
            await page.waitForTimeout(1000);


            console.log("activation link", activationLink)
            console.log("response", responseBody)
            // await assert(activationLink !== undefined)
            // Visit the activation link
            if (activationLink) {
                let url = `${BASE_URL}/callback/activate/${activationLink}`;
                console.log("url", url)
                await page.goto(url);
                await page.waitForTimeout(1000);
            }

            await page.goto(`${BASE_URL}/login`);
            await page.waitForTimeout(1000)
            await page.waitForTimeout(1000);
            await page.getByLabel('Username').fill(username);
            await page.getByLabel('Password').fill(password);
            await page.getByRole('button', {name: 'Sign In'}).click();

            // Verify redirection to home page
            await expect(page).toHaveURL(`${BASE_URL}/`);
            await expect(page.getByText('Chat Room')).toBeVisible();
        })

    })


});