// @ts-check
import {test, expect} from '@playwright/test';
import {BASE_URL, EMAIL_2, PASSWORD, USERNAME, USERNAME_2} from "../commons/constant";

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

    test('should show error for incorrect password', async ({page}) => {
        // First register a user
        await page.goto(`${BASE_URL}/signup`);
        await page.waitForTimeout(1000)
        await page.getByLabel('Username').fill(USERNAME_2);
        await page.getByLabel('Email').fill(EMAIL_2);
        await page.getByLabel('Password', {exact: true}).fill(PASSWORD);
        await page.getByLabel('Confirm Password', {exact: true}).fill(PASSWORD);
        await page.getByRole('button', {name: 'Register'}).click();

        // Try to login with wrong password
        await page.goto(`${BASE_URL}/login`);
        await page.waitForTimeout(1000)
        await page.getByLabel('Username').fill(USERNAME_2);
        await page.getByLabel('Password').fill('wrongpassword');
        await page.getByRole('button', {name: 'Sign In'}).click();

        await expect(page.getByText('Login failed, No user and password found')).toBeVisible();
    });

});