import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

WebUI.openBrowser('')

WebUI.navigateToUrl('https://login.microsoftonline.com/5962d1ad-6415-4cb2-be1f-0a8eb0738701/oauth2/authorize?client_id=00000007-0000-0000-c000-000000000000&response_mode=form_post&response_type=code+id_token&scope=openid+profile&state=OpenIdConnect.AuthenticationProperties%3dMAAAAJuRwNptVRHui8UAIkgoLxtiDR72xemKGiQtVno7Y9cddPSyadFN-yt9EaVTRhRVLAEAAAABAAAACS5yZWRpcmVjdCRodHRwczovL2lnLXNhbmRib3guY3JtLmR5bmFtaWNzLmNvbS8%26RedirectTo%3dMAAAAJuRwNptVRHui8UAIkgoLxsIpB2wm%252behUpsJJrDAl3wnWba7gA8ouEjACI7q%252bSzRnWh0dHBzOi8vaWctc2FuZGJveC5jcm0uZHluYW1pY3MuY29tLw%253d%253d%26RedirectToForMcas%3dhttps%253a%252f%252fig-sandbox.crm.dynamics.com%252f&nonce=638345035009298041.ZTk3ZTFiYmMtNDhlZi00MjBiLWI5OTMtMjQ5NWRiOWZiNzdlZjU0ZTQyOGItODY3ZC00NzFkLWJjY2EtNTNmNzY3ZWJiYjli&redirect_uri=https%3a%2f%2fbn1--namcrmlivesg645.crm.dynamics.com%2f&max_age=86400')

WebUI.setText(findTestObject('Object Repository/Page_Sign in to your account/input_Sign in_loginfmt'), 'developer@improvegroup.net')

WebUI.click(findTestObject('Object Repository/Page_Sign in to your account/input_Sign in_idSIButton9'))

WebUI.setEncryptedText(findTestObject('Object Repository/Page_Sign in to your account/input_Enter password_passwd'), 'sK5W0xXM7P4PcWMR5eMBjA==')

WebUI.click(findTestObject('Object Repository/Page_Sign in to your account/input_Sign in_idSIButton9'))

WebUI.click(findTestObject('Object Repository/Page_Sign in to your account/span_Dont show this again'))

WebUI.click(findTestObject('Object Repository/Page_Sign in to your account/input_Sign in_idSIButton9'))

WebUI.click(findTestObject('Object Repository/Page_Apps - Dynamics 365/div_Sales Team MemberTeam Member access to _92f2a0'))

WebUI.closeBrowser()

