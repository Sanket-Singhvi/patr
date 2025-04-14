/// The Deploy Details Page, has stuff like name, runner, registry, etc.
mod detail;

use self::detail::*;
use crate::prelude::*;

/// Create Deployment Page
#[component]
pub fn CreateDeploymentPage() -> impl IntoView {
	view! {
		<DeploymentPage>
			<ContainerMain class="w-full h-full">
				<ContainerHead>
					<PageTitleContainer
						page_title_items={vec![
							PageTitleItem {
								title: "Infrastructure".to_owned(),
								link: None,
								icon_position: PageTitleIconPosition::End,
								variant: PageTitleVariant::Heading,
							},
							PageTitleItem {
								title: "Deployment".to_owned(),
								link: Some("/deployment".to_owned()),
								icon_position: PageTitleIconPosition::End,
								variant: PageTitleVariant::SubHeading,
							},
							PageTitleItem {
								title: "Create Deployment".to_owned(),
								link: None,
								icon_position: PageTitleIconPosition::None,
								variant: PageTitleVariant::Text,
							},
						]}
						description_title={Some("Create a new Deployment here.".to_string())}
						description_link={Some(
							"https://docs.patr.cloud/features/deployments/#how-to-create-a-deployment"
								.to_string(),
						)}
						action_buttons={|| view! {
							<Button variant={LinkStyleVariant::Contained}>"Create Deployment"</Button>
						}}
					/>
				</ContainerHead>

				<ContainerBody>
					<DeploymentDetails />
				</ContainerBody>
			</ContainerMain>
		</DeploymentPage>
	}
}
